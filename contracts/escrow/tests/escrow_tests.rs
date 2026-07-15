//! Unit tests for the escrow contract.

#![cfg(test)]

use fusepay_escrow::{
    CreateEscrowParams, EscrowContract, EscrowContractClient, EscrowError, EscrowState,
};
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env, String,
};

// ── Test fixture ───────────────────────────────────────────────────────────

struct Fixture<'a> {
    env: Env,
    client: EscrowContractClient<'a>,
    admin: Address,
    treasury: Address,
    depositor: Address,
    beneficiary: Address,
    arbiter: Address,
    token: Address,
    token_client: TokenClient<'a>,
}

fn setup<'a>() -> Fixture<'a> {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, EscrowContract);
    let client = EscrowContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let treasury = Address::generate(&env);
    let depositor = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    let arbiter = Address::generate(&env);

    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token = token_id.address();
    let token_sac = StellarAssetClient::new(&env, &token);
    let token_client = TokenClient::new(&env, &token);

    // Mint tokens to the depositor.
    token_sac.mint(&depositor, &1_000_000_000);

    client.initialize(&admin, &treasury, &100_i128); // 100 bps = 1%

    Fixture {
        env,
        client,
        admin,
        treasury,
        depositor,
        beneficiary,
        arbiter,
        token,
        token_client,
    }
}

fn make_params(f: &Fixture) -> CreateEscrowParams {
    CreateEscrowParams {
        depositor: f.depositor.clone(),
        beneficiary: f.beneficiary.clone(),
        arbiter: f.arbiter.clone(),
        token: f.token.clone(),
        amount: 100_000_000, // 10 XLM
        timeout_ledgers: 0,  // use default
        metadata: String::from_str(&f.env, "order-001"),
    }
}

// ── Initialisation ─────────────────────────────────────────────────────────

#[test]
fn test_double_init_fails() {
    let f = setup();
    let result = f.client.try_initialize(&f.admin, &f.treasury, &100);
    assert_eq!(result, Err(Ok(EscrowError::AlreadyInitialized)));
}

// ── Create ─────────────────────────────────────────────────────────────────

#[test]
fn test_create_escrow_returns_id() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    assert_eq!(id, 1);
}

#[test]
fn test_create_sets_pending_state() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    let escrow = f.client.get_escrow(&id);
    assert_eq!(escrow.state, EscrowState::Pending);
}

#[test]
fn test_create_same_party_fails() {
    let f = setup();
    let mut params = make_params(&f);
    params.beneficiary = f.depositor.clone();
    let result = f.client.try_create_escrow(&params);
    assert_eq!(result, Err(Ok(EscrowError::SameParty)));
}

#[test]
fn test_create_below_minimum_fails() {
    let f = setup();
    let mut params = make_params(&f);
    params.amount = 1; // below 1 XLM minimum
    let result = f.client.try_create_escrow(&params);
    assert_eq!(result, Err(Ok(EscrowError::BelowMinimumAmount)));
}

// ── Fund ───────────────────────────────────────────────────────────────────

#[test]
fn test_fund_moves_to_funded_state() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    f.client.fund_escrow(&f.depositor, &id);
    let escrow = f.client.get_escrow(&id);
    assert_eq!(escrow.state, EscrowState::Funded);
}

#[test]
fn test_fund_wrong_depositor_fails() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    let other = Address::generate(&f.env);
    let result = f.client.try_fund_escrow(&other, &id);
    assert_eq!(result, Err(Ok(EscrowError::Unauthorized)));
}

#[test]
fn test_fund_twice_fails() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    f.client.fund_escrow(&f.depositor, &id);
    let result = f.client.try_fund_escrow(&f.depositor, &id);
    assert_eq!(result, Err(Ok(EscrowError::InvalidState)));
}

// ── Release ────────────────────────────────────────────────────────────────

#[test]
fn test_depositor_can_release() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    f.client.fund_escrow(&f.depositor, &id);
    f.client.release_escrow(&f.depositor, &id);

    let escrow = f.client.get_escrow(&id);
    assert_eq!(escrow.state, EscrowState::Released);
}

#[test]
fn test_release_sends_fee_to_treasury() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    f.client.fund_escrow(&f.depositor, &id);

    let treasury_before = f.token_client.balance(&f.treasury);
    f.client.release_escrow(&f.depositor, &id);
    let treasury_after = f.token_client.balance(&f.treasury);

    let escrow = f.client.get_escrow(&id);
    assert_eq!(treasury_after - treasury_before, escrow.fee_amount);
}

#[test]
fn test_beneficiary_receives_net_amount() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    f.client.fund_escrow(&f.depositor, &id);

    let before = f.token_client.balance(&f.beneficiary);
    f.client.release_escrow(&f.depositor, &id);
    let after = f.token_client.balance(&f.beneficiary);

    let escrow = f.client.get_escrow(&id);
    assert_eq!(after - before, escrow.net_amount);
}

#[test]
fn test_arbiter_can_release() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    f.client.fund_escrow(&f.depositor, &id);
    f.client.release_escrow(&f.arbiter, &id);

    let escrow = f.client.get_escrow(&id);
    assert_eq!(escrow.state, EscrowState::Released);
}

// ── Refund ─────────────────────────────────────────────────────────────────

#[test]
fn test_beneficiary_can_refund() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    f.client.fund_escrow(&f.depositor, &id);
    f.client.refund_escrow(&f.beneficiary, &id);

    let escrow = f.client.get_escrow(&id);
    assert_eq!(escrow.state, EscrowState::Refunded);
}

#[test]
fn test_refund_returns_full_amount_to_depositor() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    let before = f.token_client.balance(&f.depositor);
    f.client.fund_escrow(&f.depositor, &id);
    f.client.refund_escrow(&f.beneficiary, &id);
    let after = f.token_client.balance(&f.depositor);

    // Depositor should get the full amount back (no fee on refund).
    let escrow = f.client.get_escrow(&id);
    assert_eq!(after, before); // net zero for depositor
    let _ = escrow; // silence unused warning
}

// ── Dispute ────────────────────────────────────────────────────────────────

#[test]
fn test_depositor_can_raise_dispute() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    f.client.fund_escrow(&f.depositor, &id);
    f.client.raise_dispute(&f.depositor, &id);

    let escrow = f.client.get_escrow(&id);
    assert_eq!(escrow.state, EscrowState::Disputed);
}

#[test]
fn test_arbiter_can_resolve_release() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    f.client.fund_escrow(&f.depositor, &id);
    f.client.raise_dispute(&f.depositor, &id);
    f.client.resolve_dispute(&f.arbiter, &id, &true);

    let escrow = f.client.get_escrow(&id);
    assert_eq!(escrow.state, EscrowState::Released);
}

#[test]
fn test_arbiter_can_resolve_refund() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    f.client.fund_escrow(&f.depositor, &id);
    f.client.raise_dispute(&f.depositor, &id);
    f.client.resolve_dispute(&f.arbiter, &id, &false);

    let escrow = f.client.get_escrow(&id);
    assert_eq!(escrow.state, EscrowState::Refunded);
}

#[test]
fn test_non_arbiter_cannot_resolve() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    f.client.fund_escrow(&f.depositor, &id);
    f.client.raise_dispute(&f.depositor, &id);
    let result = f.client.try_resolve_dispute(&f.depositor, &id, &true);
    assert_eq!(result, Err(Ok(EscrowError::Unauthorized)));
}

// ── Cancel ─────────────────────────────────────────────────────────────────

#[test]
fn test_depositor_can_cancel_pending() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    f.client.cancel_escrow(&f.depositor, &id);

    let escrow = f.client.get_escrow(&id);
    assert_eq!(escrow.state, EscrowState::Cancelled);
}

#[test]
fn test_cannot_cancel_funded_escrow() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    f.client.fund_escrow(&f.depositor, &id);
    let result = f.client.try_cancel_escrow(&f.depositor, &id);
    assert_eq!(result, Err(Ok(EscrowError::InvalidState)));
}

// ── Timeout ────────────────────────────────────────────────────────────────

#[test]
fn test_timeout_refund_before_expiry_fails() {
    let f = setup();
    let id = f.client.create_escrow(&make_params(&f));
    f.client.fund_escrow(&f.depositor, &id);
    let result = f.client.try_claim_timeout_refund(&f.depositor, &id);
    assert_eq!(result, Err(Ok(EscrowError::EscrowNotExpired)));
}

#[test]
fn test_timeout_refund_after_expiry_succeeds() {
    let f = setup();
    // Use a very short timeout
    let mut params = make_params(&f);
    params.timeout_ledgers = 1;
    let id = f.client.create_escrow(&params);
    f.client.fund_escrow(&f.depositor, &id);

    // Advance ledger past expiry
    f.env.ledger().with_mut(|l| {
        l.sequence_number += 100;
    });

    f.client.claim_timeout_refund(&f.depositor, &id);
    let escrow = f.client.get_escrow(&id);
    assert_eq!(escrow.state, EscrowState::Expired);
}
