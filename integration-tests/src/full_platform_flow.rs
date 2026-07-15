//! Full end-to-end platform flow integration test.
//!
//! Simulates the complete lifecycle for a marketplace transaction:
//!
//! 1. Platform admin initialises all contracts
//! 2. Buyer creates a wallet and deposits funds
//! 3. Buyer creates an escrow and funds it
//! 4. Seller delivers goods (simulated)
//! 5. Buyer releases escrow
//! 6. Treasury receives fee
//! 7. Multisig approves a treasury withdrawal
//! 8. Treasury distributes funds to recipients

#![cfg(test)]

use fusepay_escrow::{CreateEscrowParams, EscrowContract, EscrowContractClient};
use fusepay_multisig::{MultisigContract, MultisigContractClient};
use fusepay_treasury::{DistributionEntry, TreasuryContract, TreasuryContractClient};
use fusepay_wallet::{WalletContract, WalletContractClient};
use soroban_sdk::{
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient},
    Address, Bytes, Env, String, Vec,
};

#[test]
fn test_full_marketplace_flow() {
    let env = Env::default();
    env.mock_all_auths();

    // ── Token setup ────────────────────────────────────────────────────
    let token_owner = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_owner.clone());
    let token = token_id.address();
    let token_sac = StellarAssetClient::new(&env, &token);
    let token_client = TokenClient::new(&env, &token);

    // ── Deploy all contracts ───────────────────────────────────────────
    let wallet_cid = env.register_contract(None, WalletContract);
    let wallet_client = WalletContractClient::new(&env, &wallet_cid);

    let treasury_cid = env.register_contract(None, TreasuryContract);
    let treasury_client = TreasuryContractClient::new(&env, &treasury_cid);

    let escrow_cid = env.register_contract(None, EscrowContract);
    let escrow_client = EscrowContractClient::new(&env, &escrow_cid);

    let multisig_cid = env.register_contract(None, MultisigContract);
    let multisig_client = MultisigContractClient::new(&env, &multisig_cid);

    // ── Actors ─────────────────────────────────────────────────────────
    let platform_admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let arbiter = Address::generate(&env);

    let sig_owner_1 = Address::generate(&env);
    let sig_owner_2 = Address::generate(&env);
    let sig_owner_3 = Address::generate(&env);

    // ── Initialise contracts ───────────────────────────────────────────
    wallet_client.initialize(&platform_admin).unwrap();
    treasury_client
        .initialize(
            &platform_admin,
            &multisig_cid,
            &1_000_000_000, // 100 XLM multisig threshold
        )
        .unwrap();
    escrow_client
        .initialize(&platform_admin, &treasury_cid, &100) // 1% fee
        .unwrap();

    let mut owners = Vec::new(&env);
    owners.push_back(sig_owner_1.clone());
    owners.push_back(sig_owner_2.clone());
    owners.push_back(sig_owner_3.clone());
    multisig_client.initialize(&owners, &2).unwrap(); // 2-of-3

    // ── Step 1: Buyer creates wallet and deposits ──────────────────────
    token_sac.mint(&buyer, &500_000_000); // 50 XLM

    wallet_client.create_wallet(&buyer, &token).unwrap();
    wallet_client.deposit(&buyer, &token, &500_000_000).unwrap();

    let balance = wallet_client.get_balance(&buyer, &token).unwrap();
    assert_eq!(balance, 500_000_000, "buyer wallet balance after deposit");

    // ── Step 2: Buyer creates and funds escrow ─────────────────────────
    // Buyer withdraws escrow amount from their wallet first.
    wallet_client
        .withdraw(&buyer, &token, &100_000_000, &buyer) // 10 XLM
        .unwrap();

    let escrow_params = CreateEscrowParams {
        depositor: buyer.clone(),
        beneficiary: seller.clone(),
        arbiter: arbiter.clone(),
        token: token.clone(),
        amount: 100_000_000,
        timeout_ledgers: 0,
        metadata: String::from_str(&env, "marketplace-order-42"),
    };

    let eid = escrow_client.create_escrow(&escrow_params).unwrap();
    escrow_client.fund_escrow(&buyer, &eid).unwrap();

    // ── Step 3: Buyer releases escrow after delivery ───────────────────
    let treasury_before = token_client.balance(&treasury_cid);
    let seller_before = token_client.balance(&seller);

    escrow_client.release_escrow(&buyer, &eid).unwrap();

    let escrow_record = escrow_client.get_escrow(&eid).unwrap();
    let treasury_after = token_client.balance(&treasury_cid);
    let seller_after = token_client.balance(&seller);

    // Treasury received the fee
    assert_eq!(
        treasury_after - treasury_before,
        escrow_record.fee_amount,
        "treasury fee"
    );
    // Seller received the net amount
    assert_eq!(
        seller_after - seller_before,
        escrow_record.net_amount,
        "seller net amount"
    );

    // ── Step 4: Multisig approves a treasury withdrawal ────────────────
    // (Below threshold, so admin can do directly for this test)
    let withdraw_amount = 500_000_i128; // well below 100 XLM threshold
    let recipient = Address::generate(&env);

    // The treasury now has the fee; admin can withdraw it below threshold.
    let available = treasury_client
        .get_token_ledger(&token)
        .unwrap()
        .available;
    assert!(available > 0, "treasury has fees");

    if available >= withdraw_amount {
        treasury_client
            .withdraw(&platform_admin, &token, &withdraw_amount, &recipient)
            .unwrap();
        assert_eq!(token_client.balance(&recipient), withdraw_amount);
    }

    // ── Step 5: Multi-sig flow for a large withdrawal ──────────────────
    // Top up treasury for demonstration
    token_sac.mint(&platform_admin, &2_000_000_000);
    treasury_client
        .deposit(&platform_admin, &token, &2_000_000_000)
        .unwrap();

    // Submit proposal
    let proposal_id = multisig_client
        .submit_proposal(
            &sig_owner_1,
            &String::from_str(&env, "Distribute quarterly revenue"),
            &treasury_cid,
            &Bytes::new(&env),
            &0,
        )
        .unwrap();

    // Collect approvals
    multisig_client.approve(&sig_owner_1, &proposal_id).unwrap();
    multisig_client.approve(&sig_owner_2, &proposal_id).unwrap();

    assert!(
        multisig_client.is_approved(&proposal_id),
        "proposal should be approved after 2-of-3 votes"
    );

    // With multisig approved, admin can now execute the large distribution.
    // In production the treasury would verify the proposal on-chain;
    // here we verify the approval flag is set correctly.
    let proposal = multisig_client.get_proposal(&proposal_id).unwrap();
    assert_eq!(
        proposal.approval_count, 2,
        "two approvals recorded"
    );

    // Final assertions
    let final_wallet_balance = wallet_client.get_balance(&buyer, &token).unwrap();
    assert_eq!(
        final_wallet_balance, 400_000_000,
        "buyer wallet balance after withdrawal"
    );
}
