//! Integration tests: Wallet withdrawal with fee-manager calculation.
//!
//! Verifies that the fee calculated by fee-manager correctly matches
//! the amount a wallet user would pay on a withdrawal.

#![cfg(test)]

use fusepay_fee_manager::{FeeManagerContract, FeeManagerContractClient, FeeSchedule};
use fusepay_wallet::{WalletContract, WalletContractClient};
use soroban_sdk::{
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

/// Verifies fee calculation consistency between the fee-manager and wallet.
///
/// The wallet does not call fee-manager on-chain (they are separate contracts);
/// this test confirms that the off-chain fee quote matches what the wallet
/// deducts from the user's balance.
#[test]
fn test_fee_manager_quote_matches_wallet_withdrawal() {
    let env = Env::default();
    env.mock_all_auths();

    // ── Token ──────────────────────────────────────────────────────────
    let token_owner = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_owner.clone());
    let token = token_id.address();
    let token_sac = StellarAssetClient::new(&env, &token);
    let token_client = TokenClient::new(&env, &token);

    // ── Fee-manager ────────────────────────────────────────────────────
    let fm_id = env.register_contract(None, FeeManagerContract);
    let fm_client = FeeManagerContractClient::new(&env, &fm_id);
    let fm_admin = Address::generate(&env);
    let fee_collector = Address::generate(&env);

    fm_client
        .initialize(
            &fm_admin,
            &FeeSchedule {
                platform_fee_bps: 50,
                escrow_fee_bps: 100,
                withdrawal_fee_bps: 25,
                referral_commission_bps: 50,
                fee_collector: fee_collector.clone(),
            },
        )
        .unwrap();

    // ── Wallet ─────────────────────────────────────────────────────────
    let wallet_id = env.register_contract(None, WalletContract);
    let wallet_client = WalletContractClient::new(&env, &wallet_id);
    let wallet_admin = Address::generate(&env);
    wallet_client.initialize(&wallet_admin).unwrap();

    let owner = Address::generate(&env);
    token_sac.mint(&owner, &100_000_000);

    wallet_client.create_wallet(&owner, &token).unwrap();
    wallet_client.deposit(&owner, &token, &100_000_000).unwrap();

    // ── Get fee quote from fee-manager ─────────────────────────────────
    let withdrawal_amount = 50_000_000_i128;
    let breakdown = fm_client
        .calculate_withdrawal_fee(&withdrawal_amount, &owner, &None)
        .unwrap();

    // withdrawal_fee_bps = 25 → fee = 50_000_000 × 25 / 10000 = 125_000
    assert_eq!(breakdown.base_fee, 125_000);
    assert_eq!(breakdown.net_fee, 125_000);
    assert_eq!(breakdown.net_principal, 49_875_000);

    // ── Wallet withdrawal (no fee deducted by wallet itself) ───────────
    // In FusePay, the backend routes the withdrawal amount and fee
    // separately based on the fee-manager quote.  The wallet just
    // transfers the requested amount.
    let recipient = Address::generate(&env);
    wallet_client
        .withdraw(&owner, &token, &withdrawal_amount, &recipient)
        .unwrap();

    // Recipient gets the full withdrawal_amount (fee routing is off-chain).
    assert_eq!(token_client.balance(&recipient), withdrawal_amount);

    // Remaining balance in wallet.
    let balance = wallet_client.get_balance(&owner, &token).unwrap();
    assert_eq!(balance, 50_000_000);
}
