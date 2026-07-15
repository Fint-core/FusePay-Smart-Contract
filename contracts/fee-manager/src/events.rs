//! Event emission for the fee-manager contract.

use soroban_sdk::{symbol_short, Address, Env, Symbol};

fn topic(env: &Env) -> Symbol {
    Symbol::new(env, "fp_feemgr")
}

/// Emitted when the fee schedule is updated.
pub fn emit_fee_schedule_updated(
    env: &Env,
    updated_by: &Address,
    platform_fee_bps: i128,
    escrow_fee_bps: i128,
    withdrawal_fee_bps: i128,
) {
    env.events().publish(
        (topic(env), symbol_short!("sched_upd"), updated_by.clone()),
        (
            platform_fee_bps,
            escrow_fee_bps,
            withdrawal_fee_bps,
            env.ledger().sequence(),
        ),
    );
}

/// Emitted when a discount rule is set for an address.
pub fn emit_discount_set(
    env: &Env,
    address: &Address,
    discount_bps: i128,
    expires_at_ledger: u32,
    set_by: &Address,
) {
    env.events().publish(
        (topic(env), symbol_short!("disc_set"), address.clone()),
        (
            discount_bps,
            expires_at_ledger,
            set_by.clone(),
            env.ledger().sequence(),
        ),
    );
}

/// Emitted when a discount rule is removed.
pub fn emit_discount_removed(env: &Env, address: &Address, removed_by: &Address) {
    env.events().publish(
        (topic(env), symbol_short!("disc_rem"), address.clone()),
        (removed_by.clone(), env.ledger().sequence()),
    );
}

/// Emitted when a fee is collected (called by external contracts).
pub fn emit_fee_collected(
    env: &Env,
    payer: &Address,
    token: &Address,
    net_fee: i128,
    referral_commission: i128,
) {
    env.events().publish(
        (topic(env), symbol_short!("fee_coll"), payer.clone()),
        (
            token.clone(),
            net_fee,
            referral_commission,
            env.ledger().sequence(),
        ),
    );
}
