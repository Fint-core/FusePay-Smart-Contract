//! Unit tests for the wallet contract.

#![cfg(test)]

use fusepay_wallet::{WalletContract, WalletContractClient, WalletError};
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env, IntoVal,
};

// ── Test helpers ───────────────────────────────────────────────────────────

struct TestFixture<'a> {
    env: Env,
    client: WalletContractClient<'a>,
    admin: Address,
    owner: Address,
    token: Address,
    token_client: TokenClient<'a>,
}

fn setup<'a>() -> TestFixture<'a> {
    let env = Env::default();
    env.mock_all_auths();

    // Deploy wallet contract
    let contract_id = env.register_contract(None, WalletContract);
    let client = WalletContractClient::new(&env, &contract_id);

    // Deploy a Stellar Asset token for testing
    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token = token_id.address();
    let token_sac = StellarAssetClient::new(&env, &token);
    let token_client = TokenClient::new(&env, &token);

    let owner = Address::generate(&env);

    // Mint some tokens to the owner for testing
    token_sac.mint(&owner, &1_000_000_000);

    // Initialise the wallet contract
    client.initialize(&admin);

    TestFixture {
        env,
        client,
        admin,
        owner,
        token,
        token_client,
    }
}

// ── Initialisation ─────────────────────────────────────────────────────────

#[test]
fn test_initialize_succeeds() {
    let f = setup();
    // If we reach here without panic, initialisation succeeded.
    // Verify by trying to initialise again — should fail.
    let result = f.client.try_initialize(&f.admin);
    assert_eq!(result, Err(Ok(WalletError::AlreadyInitialized)));
}

// ── Wallet creation ────────────────────────────────────────────────────────

#[test]
fn test_create_wallet_succeeds() {
    let f = setup();
    f.client.create_wallet(&f.owner, &f.token);
    let wallet = f.client.get_wallet(&f.owner, &f.token);
    assert_eq!(wallet.owner, f.owner);
    assert_eq!(wallet.balance, 0);
    assert!(wallet.is_active);
}

#[test]
fn test_create_wallet_duplicate_fails() {
    let f = setup();
    f.client.create_wallet(&f.owner, &f.token);
    let result = f.client.try_create_wallet(&f.owner, &f.token);
    assert_eq!(result, Err(Ok(WalletError::WalletAlreadyExists)));
}

// ── Deposits ───────────────────────────────────────────────────────────────

#[test]
fn test_deposit_increases_balance() {
    let f = setup();
    f.client.create_wallet(&f.owner, &f.token);
    f.client.deposit(&f.owner, &f.token, &10_000_000);

    let balance = f.client.get_balance(&f.owner, &f.token);
    assert_eq!(balance, 10_000_000);
}

#[test]
fn test_multiple_deposits_accumulate() {
    let f = setup();
    f.client.create_wallet(&f.owner, &f.token);
    f.client.deposit(&f.owner, &f.token, &10_000_000);
    f.client.deposit(&f.owner, &f.token, &5_000_000);

    let wallet = f.client.get_wallet(&f.owner, &f.token);
    assert_eq!(wallet.balance, 15_000_000);
    assert_eq!(wallet.total_deposited, 15_000_000);
}

#[test]
fn test_deposit_zero_fails() {
    let f = setup();
    f.client.create_wallet(&f.owner, &f.token);
    let result = f.client.try_deposit(&f.owner, &f.token, &0);
    assert_eq!(result, Err(Ok(WalletError::InvalidAmount)));
}

#[test]
fn test_deposit_negative_fails() {
    let f = setup();
    f.client.create_wallet(&f.owner, &f.token);
    let result = f.client.try_deposit(&f.owner, &f.token, &(-1));
    assert_eq!(result, Err(Ok(WalletError::InvalidAmount)));
}

// ── Withdrawals ────────────────────────────────────────────────────────────

#[test]
fn test_withdrawal_decreases_balance() {
    let f = setup();
    let recipient = Address::generate(&f.env);
    f.client.create_wallet(&f.owner, &f.token);
    f.client.deposit(&f.owner, &f.token, &20_000_000);
    f.client.withdraw(&f.owner, &f.token, &5_000_000, &recipient);

    let balance = f.client.get_balance(&f.owner, &f.token);
    assert_eq!(balance, 15_000_000);
}

#[test]
fn test_withdrawal_updates_total_withdrawn() {
    let f = setup();
    let recipient = Address::generate(&f.env);
    f.client.create_wallet(&f.owner, &f.token);
    f.client.deposit(&f.owner, &f.token, &20_000_000);
    f.client.withdraw(&f.owner, &f.token, &5_000_000, &recipient);

    let wallet = f.client.get_wallet(&f.owner, &f.token);
    assert_eq!(wallet.total_withdrawn, 5_000_000);
}

#[test]
fn test_overdraft_fails() {
    let f = setup();
    let recipient = Address::generate(&f.env);
    f.client.create_wallet(&f.owner, &f.token);
    f.client.deposit(&f.owner, &f.token, &5_000_000);

    let result = f
        .client
        .try_withdraw(&f.owner, &f.token, &10_000_000, &recipient);
    assert_eq!(result, Err(Ok(WalletError::InsufficientFunds)));
}

#[test]
fn test_withdrawal_zero_fails() {
    let f = setup();
    let recipient = Address::generate(&f.env);
    f.client.create_wallet(&f.owner, &f.token);
    let result = f.client.try_withdraw(&f.owner, &f.token, &0, &recipient);
    assert_eq!(result, Err(Ok(WalletError::InvalidAmount)));
}

// ── Freeze / unfreeze ──────────────────────────────────────────────────────

#[test]
fn test_admin_can_freeze_wallet() {
    let f = setup();
    f.client.create_wallet(&f.owner, &f.token);
    f.client.freeze_wallet(&f.admin, &f.owner, &f.token);

    let wallet = f.client.get_wallet(&f.owner, &f.token);
    assert!(!wallet.is_active);
}

#[test]
fn test_deposit_on_frozen_wallet_fails() {
    let f = setup();
    f.client.create_wallet(&f.owner, &f.token);
    f.client.freeze_wallet(&f.admin, &f.owner, &f.token);

    let result = f.client.try_deposit(&f.owner, &f.token, &10_000_000);
    assert_eq!(result, Err(Ok(WalletError::WalletFrozen)));
}

#[test]
fn test_withdrawal_on_frozen_wallet_fails() {
    let f = setup();
    let recipient = Address::generate(&f.env);
    f.client.create_wallet(&f.owner, &f.token);
    f.client.deposit(&f.owner, &f.token, &20_000_000);
    f.client.freeze_wallet(&f.admin, &f.owner, &f.token);

    let result = f
        .client
        .try_withdraw(&f.owner, &f.token, &5_000_000, &recipient);
    assert_eq!(result, Err(Ok(WalletError::WalletFrozen)));
}

#[test]
fn test_admin_can_unfreeze_wallet() {
    let f = setup();
    f.client.create_wallet(&f.owner, &f.token);
    f.client.freeze_wallet(&f.admin, &f.owner, &f.token);
    f.client.unfreeze_wallet(&f.admin, &f.owner, &f.token);

    let wallet = f.client.get_wallet(&f.owner, &f.token);
    assert!(wallet.is_active);
}

#[test]
fn test_non_admin_cannot_freeze_wallet() {
    let f = setup();
    let random = Address::generate(&f.env);
    f.client.create_wallet(&f.owner, &f.token);
    let result = f.client.try_freeze_wallet(&random, &f.owner, &f.token);
    assert_eq!(result, Err(Ok(WalletError::Unauthorized)));
}

// ── Security edge cases ────────────────────────────────────────────────────

#[test]
fn test_token_mismatch_on_deposit_fails() {
    let f = setup();
    let other_token = Address::generate(&f.env);
    f.client.create_wallet(&f.owner, &f.token);
    let result = f.client.try_deposit(&f.owner, &other_token, &10_000_000);
    // The wallet was created with f.token, so other_token should be rejected.
    // Note: In a real env the lookup by (owner, other_token) would return None → NotInitialized.
    assert!(result.is_err());
}
