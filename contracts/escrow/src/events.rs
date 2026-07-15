//! Event emission for the escrow contract.

use soroban_sdk::{symbol_short, Address, Env, Symbol};

fn topic(env: &Env) -> Symbol {
    Symbol::new(env, "fp_escrow")
}

/// Emitted when a new escrow is created.
///
/// # Topics: `(fp_escrow, created, depositor)`
/// # Data: `(escrow_id: u64, beneficiary: Address, token: Address, amount: i128, expires_at: u32)`
pub fn emit_escrow_created(
    env: &Env,
    escrow_id: u64,
    depositor: &Address,
    beneficiary: &Address,
    token: &Address,
    amount: i128,
    expires_at_ledger: u32,
) {
    env.events().publish(
        (topic(env), symbol_short!("created"), depositor.clone()),
        (
            escrow_id,
            beneficiary.clone(),
            token.clone(),
            amount,
            expires_at_ledger,
        ),
    );
}

/// Emitted when funds are deposited into an escrow.
///
/// # Topics: `(fp_escrow, funded, depositor)`
/// # Data: `(escrow_id: u64, amount: i128, ledger: u32)`
pub fn emit_escrow_funded(env: &Env, escrow_id: u64, depositor: &Address, amount: i128) {
    env.events().publish(
        (topic(env), symbol_short!("funded"), depositor.clone()),
        (escrow_id, amount, env.ledger().sequence()),
    );
}

/// Emitted when escrow funds are released to the beneficiary.
///
/// # Topics: `(fp_escrow, released, beneficiary)`
/// # Data: `(escrow_id: u64, net_amount: i128, fee_amount: i128, released_by: Address)`
pub fn emit_escrow_released(
    env: &Env,
    escrow_id: u64,
    beneficiary: &Address,
    net_amount: i128,
    fee_amount: i128,
    released_by: &Address,
) {
    env.events().publish(
        (topic(env), symbol_short!("released"), beneficiary.clone()),
        (escrow_id, net_amount, fee_amount, released_by.clone()),
    );
}

/// Emitted when escrow funds are refunded to the depositor.
///
/// # Topics: `(fp_escrow, refunded, depositor)`
/// # Data: `(escrow_id: u64, amount: i128, refunded_by: Address, ledger: u32)`
pub fn emit_escrow_refunded(
    env: &Env,
    escrow_id: u64,
    depositor: &Address,
    amount: i128,
    refunded_by: &Address,
) {
    env.events().publish(
        (topic(env), symbol_short!("refunded"), depositor.clone()),
        (
            escrow_id,
            amount,
            refunded_by.clone(),
            env.ledger().sequence(),
        ),
    );
}

/// Emitted when a dispute is raised on an escrow.
///
/// # Topics: `(fp_escrow, disputed, raised_by)`
/// # Data: `(escrow_id: u64, ledger: u32)`
pub fn emit_dispute_raised(env: &Env, escrow_id: u64, raised_by: &Address) {
    env.events().publish(
        (topic(env), symbol_short!("disputed"), raised_by.clone()),
        (escrow_id, env.ledger().sequence()),
    );
}

/// Emitted when an escrow is cancelled before funding.
///
/// # Topics: `(fp_escrow, cancelled, depositor)`
/// # Data: `(escrow_id: u64, ledger: u32)`
pub fn emit_escrow_cancelled(env: &Env, escrow_id: u64, cancelled_by: &Address) {
    env.events().publish(
        (topic(env), symbol_short!("canceld"), cancelled_by.clone()),
        (escrow_id, env.ledger().sequence()),
    );
}

/// Emitted when an escrow times out and is auto-refunded.
///
/// # Topics: `(fp_escrow, expired, depositor)`
/// # Data: `(escrow_id: u64, amount: i128, ledger: u32)`
pub fn emit_escrow_expired(env: &Env, escrow_id: u64, depositor: &Address, amount: i128) {
    env.events().publish(
        (topic(env), symbol_short!("expired"), depositor.clone()),
        (escrow_id, amount, env.ledger().sequence()),
    );
}
