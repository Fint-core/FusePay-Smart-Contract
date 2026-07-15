//! Integration tests: Escrow → Treasury fee flow.
//!
//! Verifies that fees collected by the escrow contract are correctly
//! received by the treasury contract.

#![cfg(test)]

use fusepay_escrow::{CreateEscrowParams, EscrowContract, EscrowContractClient};
use fusepay_treasury::{TreasuryContract, TreasuryContractClient};
use soroban_sdk::{
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env, String,
};

/// Full escrow-release flow with fee routing to treasury.
///
/// 1. Deploy treasury and escrow contracts
/// 2. Create and fund an escrow
/// 3. Release the escrow
/// 4. Assert treasury balance increased by exactly the fee amount
#[test]
fn test_escrow_release_deposits_fee_to_treasury() {
    let env = Env::default();
    env.mock_all_auths();

    // ── Deploy token ───────────────────────────────────────────────────
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token = token_id.address();
    let token_sac = StellarAssetClient::new(&env, &token);
    let token_client = TokenClient::new(&env, &token);

    // ── Deploy treasury ────────────────────────────────────────────────
    let treasury_id = env.register_contract(None, TreasuryContract);
    let treasury_client = TreasuryContractClient::new(&env, &treasury_id);
    let treasury_admin = Address::generate(&env);
    let multisig_addr = Address::generate(&env);
    treasury_client
        .initialize(&treasury_admin, &multisig_addr, &1_000_000_000)
        .unwrap();

    // ── Deploy escrow with treasury as fee recipient ───────────────────
    let escrow_id = env.register_contract(None, EscrowContract);
    let escrow_client = EscrowContractClient::new(&env, &escrow_id);
    let escrow_admin = Address::generate(&env);
    // fee_bps = 100 (1%)
    escrow_client
        .initialize(&escrow_admin, &treasury_id, &100)
        .unwrap();

    // ── Actors ─────────────────────────────────────────────────────────
    let depositor = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    let arbiter = Address::generate(&env);

    // Mint 10 XLM to depositor
    token_sac.mint(&depositor, &100_000_000);

    // ── Create and fund escrow ─────────────────────────────────────────
    let params = CreateEscrowParams {
        depositor: depositor.clone(),
        beneficiary: beneficiary.clone(),
        arbiter: arbiter.clone(),
        token: token.clone(),
        amount: 100_000_000, // 10 XLM
        timeout_ledgers: 0,
        metadata: String::from_str(&env, "integration-test-001"),
    };

    let escrow_id_num = escrow_client.create_escrow(&params).unwrap();
    escrow_client.fund_escrow(&depositor, &escrow_id_num).unwrap();

    // ── Capture treasury balance before release ────────────────────────
    let treasury_before = token_client.balance(&treasury_id);

    // ── Release escrow ─────────────────────────────────────────────────
    escrow_client
        .release_escrow(&depositor, &escrow_id_num)
        .unwrap();

    // ── Assert treasury received the fee ──────────────────────────────
    let treasury_after = token_client.balance(&treasury_id);
    let escrow_record = escrow_client.get_escrow(&escrow_id_num).unwrap();

    assert_eq!(
        treasury_after - treasury_before,
        escrow_record.fee_amount,
        "treasury should have received exactly the escrow fee"
    );

    // ── Assert beneficiary received net amount ─────────────────────────
    let beneficiary_balance = token_client.balance(&beneficiary);
    assert_eq!(
        beneficiary_balance,
        escrow_record.net_amount,
        "beneficiary should receive net amount"
    );

    // fee + net = total
    assert_eq!(
        escrow_record.fee_amount + escrow_record.net_amount,
        100_000_000,
        "fee + net must equal the locked amount"
    );
}

/// Escrow refund should not deposit any fee to treasury.
#[test]
fn test_escrow_refund_sends_nothing_to_treasury() {
    let env = Env::default();
    env.mock_all_auths();

    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token = token_id.address();
    let token_sac = StellarAssetClient::new(&env, &token);
    let token_client = TokenClient::new(&env, &token);

    let treasury_id = env.register_contract(None, TreasuryContract);
    let treasury_client = TreasuryContractClient::new(&env, &treasury_id);
    let treasury_admin = Address::generate(&env);
    let multisig_addr = Address::generate(&env);
    treasury_client
        .initialize(&treasury_admin, &multisig_addr, &1_000_000_000)
        .unwrap();

    let escrow_contract_id = env.register_contract(None, EscrowContract);
    let escrow_client = EscrowContractClient::new(&env, &escrow_contract_id);
    let escrow_admin = Address::generate(&env);
    escrow_client
        .initialize(&escrow_admin, &treasury_id, &100)
        .unwrap();

    let depositor = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    let arbiter = Address::generate(&env);

    token_sac.mint(&depositor, &100_000_000);

    let params = CreateEscrowParams {
        depositor: depositor.clone(),
        beneficiary: beneficiary.clone(),
        arbiter,
        token: token.clone(),
        amount: 100_000_000,
        timeout_ledgers: 0,
        metadata: String::from_str(&env, "refund-test"),
    };

    let eid = escrow_client.create_escrow(&params).unwrap();
    escrow_client.fund_escrow(&depositor, &eid).unwrap();

    let treasury_before = token_client.balance(&treasury_id);
    escrow_client.refund_escrow(&beneficiary, &eid).unwrap();
    let treasury_after = token_client.balance(&treasury_id);

    // No fee on refund
    assert_eq!(treasury_after, treasury_before, "no fee on refund");

    // Depositor gets full amount back
    assert_eq!(token_client.balance(&depositor), 100_000_000);
}
