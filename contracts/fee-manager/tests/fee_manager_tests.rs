//! Unit tests for the fee-manager contract.

#![cfg(test)]

use fusepay_fee_manager::{
    DiscountRule, FeeManagerContract, FeeManagerContractClient, FeeManagerError, FeeSchedule,
};
use soroban_sdk::{testutils::{Address as _, Ledger as _}, Address, Env};

// ── Helpers ────────────────────────────────────────────────────────────────

fn default_schedule(env: &Env, collector: &Address) -> FeeSchedule {
    FeeSchedule {
        platform_fee_bps: 50,       // 0.5%
        escrow_fee_bps: 100,        // 1.0%
        withdrawal_fee_bps: 25,     // 0.25%
        referral_commission_bps: 50, // 0.5% of net fee
        fee_collector: collector.clone(),
    }
}

fn setup(env: &Env) -> (FeeManagerContractClient, Address, Address) {
    let contract_id = env.register_contract(None, FeeManagerContract);
    let client = FeeManagerContractClient::new(env, &contract_id);
    let admin = Address::generate(env);
    let collector = Address::generate(env);
    client.initialize(&admin, &default_schedule(env, &collector));
    (client, admin, collector)
}

// ── Initialisation ─────────────────────────────────────────────────────────

#[test]
fn test_double_init_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin, collector) = setup(&env);
    let result = client.try_initialize(&admin, &default_schedule(&env, &collector));
    assert_eq!(result, Err(Ok(FeeManagerError::AlreadyInitialized)));
}

#[test]
fn test_init_fee_too_high_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, FeeManagerContract);
    let client = FeeManagerContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let collector = Address::generate(&env);

    let mut schedule = default_schedule(&env, &collector);
    schedule.platform_fee_bps = 600; // > MAX_PLATFORM_FEE_BPS (500)

    let result = client.try_initialize(&admin, &schedule);
    assert_eq!(result, Err(Ok(FeeManagerError::FeeTooHigh)));
}

// ── Platform fee calculation ───────────────────────────────────────────────

#[test]
fn test_calculate_platform_fee_standard() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _, _) = setup(&env);
    let payer = Address::generate(&env);

    // 1,000,000 stroops × 50 bps = 5,000 fee
    let breakdown = client.calculate_platform_fee(&1_000_000, &payer, &None);

    assert_eq!(breakdown.principal, 1_000_000);
    assert_eq!(breakdown.base_fee, 5_000);
    assert_eq!(breakdown.discount_amount, 0);
    assert_eq!(breakdown.net_fee, 5_000);
    assert_eq!(breakdown.net_principal, 995_000);
    assert_eq!(breakdown.referral_commission, 0);
}

#[test]
fn test_calculate_fee_with_referrer() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _, _) = setup(&env);
    let payer = Address::generate(&env);
    let referrer = Address::generate(&env);

    // 1,000,000 × 50 bps = 5,000 fee
    // 50 bps commission on 5,000 = 25 commission
    let breakdown = client.calculate_platform_fee(&1_000_000, &payer, &Some(referrer));

    assert_eq!(breakdown.net_fee, 5_000);
    assert_eq!(breakdown.referral_commission, 25); // 50 bps of 5,000
}

#[test]
fn test_self_referral_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _, _) = setup(&env);
    let payer = Address::generate(&env);

    let result = client.try_calculate_platform_fee(&1_000_000, &payer, &Some(payer.clone()));
    assert_eq!(result, Err(Ok(FeeManagerError::SelfReferral)));
}

// ── Discount rules ─────────────────────────────────────────────────────────

#[test]
fn test_set_discount_applies_to_fee() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin, _) = setup(&env);
    let payer = Address::generate(&env);

    // 50% discount on the fee
    let rule = DiscountRule {
        address: payer.clone(),
        discount_bps: 5_000, // 50%
        expires_at_ledger: 0, // never expires
    };
    client.set_discount(&admin, &rule);

    // 1,000,000 × 50 bps = 5,000 base fee
    // 50% discount = 2,500 discount
    // net fee = 2,500
    let breakdown = client.calculate_platform_fee(&1_000_000, &payer, &None);

    assert_eq!(breakdown.base_fee, 5_000);
    assert_eq!(breakdown.discount_amount, 2_500);
    assert_eq!(breakdown.net_fee, 2_500);
    assert_eq!(breakdown.net_principal, 997_500);
}

#[test]
fn test_100_percent_discount_zeroes_fee() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin, _) = setup(&env);
    let payer = Address::generate(&env);

    let rule = DiscountRule {
        address: payer.clone(),
        discount_bps: 10_000, // 100% discount
        expires_at_ledger: 0,
    };
    client.set_discount(&admin, &rule);

    let breakdown = client.calculate_platform_fee(&1_000_000, &payer, &None);

    assert_eq!(breakdown.net_fee, 0);
    assert_eq!(breakdown.net_principal, 1_000_000);
}

#[test]
fn test_expired_discount_not_applied() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin, _) = setup(&env);
    let payer = Address::generate(&env);

    let rule = DiscountRule {
        address: payer.clone(),
        discount_bps: 5_000,
        expires_at_ledger: 1, // already expired at ledger 1
    };
    client.set_discount(&admin, &rule);

    // Advance ledger past expiry
    env.ledger().with_mut(|l| {
        l.sequence_number = 100;
    });

    let breakdown = client.calculate_platform_fee(&1_000_000, &payer, &None);

    // Expired discount should not apply
    assert_eq!(breakdown.discount_amount, 0);
    assert_eq!(breakdown.net_fee, 5_000);
}

#[test]
fn test_remove_discount_succeeds() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin, _) = setup(&env);
    let payer = Address::generate(&env);

    let rule = DiscountRule {
        address: payer.clone(),
        discount_bps: 5_000,
        expires_at_ledger: 0,
    };
    client.set_discount(&admin, &rule);
    client.remove_discount(&admin, &payer);

    assert!(client.get_discount(&payer).is_none());
}

#[test]
fn test_remove_nonexistent_discount_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin, _) = setup(&env);
    let addr = Address::generate(&env);

    let result = client.try_remove_discount(&admin, &addr);
    assert_eq!(result, Err(Ok(FeeManagerError::DiscountNotFound)));
}

#[test]
fn test_invalid_discount_bps_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin, _) = setup(&env);
    let payer = Address::generate(&env);

    let rule = DiscountRule {
        address: payer.clone(),
        discount_bps: 10_001, // > 100%
        expires_at_ledger: 0,
    };
    let result = client.try_set_discount(&admin, &rule);
    assert_eq!(result, Err(Ok(FeeManagerError::InvalidDiscount)));
}

// ── Fee types ──────────────────────────────────────────────────────────────

#[test]
fn test_escrow_fee_uses_escrow_rate() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _, _) = setup(&env);
    let payer = Address::generate(&env);

    // escrow_fee_bps = 100 (1%)
    let breakdown = client.calculate_escrow_fee(&1_000_000, &payer, &None);

    assert_eq!(breakdown.base_fee, 10_000); // 1% of 1,000,000
}

#[test]
fn test_withdrawal_fee_uses_withdrawal_rate() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _, _) = setup(&env);
    let payer = Address::generate(&env);

    // withdrawal_fee_bps = 25 (0.25%)
    let breakdown = client.calculate_withdrawal_fee(&1_000_000, &payer, &None);

    assert_eq!(breakdown.base_fee, 2_500); // 0.25% of 1,000,000
}

// ── Update schedule ────────────────────────────────────────────────────────

#[test]
fn test_admin_can_update_schedule() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin, collector) = setup(&env);

    let new_schedule = FeeSchedule {
        platform_fee_bps: 30,
        escrow_fee_bps: 75,
        withdrawal_fee_bps: 10,
        referral_commission_bps: 20,
        fee_collector: collector.clone(),
    };

    client.update_fee_schedule(&admin, &new_schedule);
    let stored = client.get_fee_schedule();
    assert_eq!(stored.platform_fee_bps, 30);
}

#[test]
fn test_non_admin_cannot_update_schedule() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _, collector) = setup(&env);
    let random = Address::generate(&env);

    let result = client.try_update_fee_schedule(&random, &default_schedule(&env, &collector));
    assert_eq!(result, Err(Ok(FeeManagerError::Unauthorized)));
}
