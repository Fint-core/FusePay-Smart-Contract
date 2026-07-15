//! Unit tests for the treasury contract.

#![cfg(test)]

use fusepay_treasury::{
    DistributionEntry, TreasuryContract, TreasuryContractClient, TreasuryError,
};
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env, Vec,
};

// ── Fixture ────────────────────────────────────────────────────────────────

struct Fixture<'a> {
    env: Env,
    client: TreasuryContractClient<'a>,
    admin: Address,
    multisig: Address,
    depositor: Address,
    token: Address,
    token_client: TokenClient<'a>,
}

/// multisig_threshold = 100 XLM (1_000_000_000 stroops)
const MULTISIG_THRESHOLD: i128 = 1_000_000_000;

fn setup<'a>() -> Fixture<'a> {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, TreasuryContract);
    let client = TreasuryContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let multisig = Address::generate(&env);
    let depositor = Address::generate(&env);

    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token = token_id.address();
    let token_sac = StellarAssetClient::new(&env, &token);
    let token_client = TokenClient::new(&env, &token);

    token_sac.mint(&depositor, &10_000_000_000);

    client.initialize(&admin, &multisig, &MULTISIG_THRESHOLD);

    Fixture {
        env,
        client,
        admin,
        multisig,
        depositor,
        token,
        token_client,
    }
}

// ── Initialisation ─────────────────────────────────────────────────────────

#[test]
fn test_double_init_fails() {
    let f = setup();
    let result = f.client.try_initialize(&f.admin, &f.multisig, &MULTISIG_THRESHOLD);
    assert_eq!(result, Err(Ok(TreasuryError::AlreadyInitialized)));
}

// ── Deposit ────────────────────────────────────────────────────────────────

#[test]
fn test_deposit_increases_available_balance() {
    let f = setup();
    f.client.deposit(&f.depositor, &f.token, &500_000_000);

    let ledger = f.client.get_token_ledger(&f.token);
    assert_eq!(ledger.available, 500_000_000);
    assert_eq!(ledger.total_deposited, 500_000_000);
}

#[test]
fn test_multiple_deposits_accumulate() {
    let f = setup();
    f.client.deposit(&f.depositor, &f.token, &200_000_000);
    f.client.deposit(&f.depositor, &f.token, &300_000_000);

    let ledger = f.client.get_token_ledger(&f.token);
    assert_eq!(ledger.available, 500_000_000);
    assert_eq!(ledger.total_deposited, 500_000_000);
}

#[test]
fn test_deposit_zero_fails() {
    let f = setup();
    let result = f.client.try_deposit(&f.depositor, &f.token, &0);
    assert_eq!(result, Err(Ok(TreasuryError::ZeroAmount)));
}

// ── Withdraw ───────────────────────────────────────────────────────────────

#[test]
fn test_admin_withdraw_below_threshold() {
    let f = setup();
    let recipient = Address::generate(&f.env);
    f.client.deposit(&f.depositor, &f.token, &500_000_000);

    // 50 XLM withdrawal — below the 100 XLM threshold
    f.client.withdraw(&f.admin, &f.token, &(500_000_000 - 1), &recipient);

    let ledger = f.client.get_token_ledger(&f.token);
    assert_eq!(ledger.available, 1);
}

#[test]
fn test_withdraw_at_threshold_requires_multisig() {
    let f = setup();
    let recipient = Address::generate(&f.env);
    f.client.deposit(&f.depositor, &f.token, &2_000_000_000);

    let result = f
        .client
        .try_withdraw(&f.admin, &f.token, &MULTISIG_THRESHOLD, &recipient);
    assert_eq!(result, Err(Ok(TreasuryError::RequiresMultisig)));
}

#[test]
fn test_withdraw_above_threshold_requires_multisig() {
    let f = setup();
    let recipient = Address::generate(&f.env);
    f.client.deposit(&f.depositor, &f.token, &2_000_000_000);

    let result = f
        .client
        .try_withdraw(&f.admin, &f.token, &(MULTISIG_THRESHOLD + 1), &recipient);
    assert_eq!(result, Err(Ok(TreasuryError::RequiresMultisig)));
}

#[test]
fn test_withdraw_exceeds_balance_fails() {
    let f = setup();
    let recipient = Address::generate(&f.env);
    f.client.deposit(&f.depositor, &f.token, &100_000_000);

    let result = f
        .client
        .try_withdraw(&f.admin, &f.token, &200_000_000, &recipient);
    assert_eq!(result, Err(Ok(TreasuryError::InsufficientBalance)));
}

#[test]
fn test_non_admin_cannot_withdraw() {
    let f = setup();
    let random = Address::generate(&f.env);
    let recipient = Address::generate(&f.env);
    f.client.deposit(&f.depositor, &f.token, &100_000_000);

    let result = f.client.try_withdraw(&random, &f.token, &50_000_000, &recipient);
    assert_eq!(result, Err(Ok(TreasuryError::Unauthorized)));
}

// ── Distribute ─────────────────────────────────────────────────────────────

#[test]
fn test_distribute_to_multiple_recipients() {
    let f = setup();
    let r1 = Address::generate(&f.env);
    let r2 = Address::generate(&f.env);

    f.client.deposit(&f.depositor, &f.token, &500_000_000);

    let mut entries = Vec::new(&f.env);
    entries.push_back(DistributionEntry {
        recipient: r1.clone(),
        amount: 200_000_000,
    });
    entries.push_back(DistributionEntry {
        recipient: r2.clone(),
        amount: 100_000_000,
    });

    f.client.distribute(&f.admin, &f.token, &entries);

    assert_eq!(f.token_client.balance(&r1), 200_000_000);
    assert_eq!(f.token_client.balance(&r2), 100_000_000);

    let ledger = f.client.get_token_ledger(&f.token);
    assert_eq!(ledger.available, 200_000_000);
    assert_eq!(ledger.total_distributed, 300_000_000);
}

#[test]
fn test_distribute_empty_list_fails() {
    let f = setup();
    let entries: Vec<DistributionEntry> = Vec::new(&f.env);
    let result = f.client.try_distribute(&f.admin, &f.token, &entries);
    assert_eq!(result, Err(Ok(TreasuryError::InvalidDistribution)));
}

// ── Threshold update ───────────────────────────────────────────────────────

#[test]
fn test_admin_can_update_threshold() {
    let f = setup();
    f.client.update_multisig_threshold(&f.admin, &500_000_000);

    let config = f.client.get_config();
    assert_eq!(config.multisig_threshold, 500_000_000);
}

#[test]
fn test_accounting_totals_are_correct() {
    let f = setup();
    let recipient = Address::generate(&f.env);

    f.client.deposit(&f.depositor, &f.token, &500_000_000);
    f.client.deposit(&f.depositor, &f.token, &200_000_000);
    f.client.withdraw(&f.admin, &f.token, &100_000_000, &recipient);

    let ledger = f.client.get_token_ledger(&f.token);
    assert_eq!(ledger.total_deposited, 700_000_000);
    assert_eq!(ledger.total_distributed, 100_000_000);
    assert_eq!(ledger.available, 600_000_000);
}
