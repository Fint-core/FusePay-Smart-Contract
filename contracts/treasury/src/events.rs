//! Event emission for the treasury contract.

use soroban_sdk::{symbol_short, Address, Env, Symbol};

fn topic(env: &Env) -> Symbol {
    Symbol::new(env, "fp_treasury")
}

/// Emitted when funds are deposited into the treasury.
///
/// # Topics: `(fp_treasury, deposit, token)`
/// # Data: `(amount: i128, sender: Address, new_balance: i128, ledger: u32)`
pub fn emit_treasury_deposit(
    env: &Env,
    token: &Address,
    amount: i128,
    sender: &Address,
    new_balance: i128,
) {
    env.events().publish(
        (topic(env), symbol_short!("deposit"), token.clone()),
        (amount, sender.clone(), new_balance, env.ledger().sequence()),
    );
}

/// Emitted when funds are withdrawn from the treasury.
///
/// # Topics: `(fp_treasury, withdraw, token)`
/// # Data: `(amount: i128, recipient: Address, withdrawn_by: Address, new_balance: i128, ledger: u32)`
pub fn emit_treasury_withdraw(
    env: &Env,
    token: &Address,
    amount: i128,
    recipient: &Address,
    withdrawn_by: &Address,
    new_balance: i128,
) {
    env.events().publish(
        (topic(env), symbol_short!("withdraw"), token.clone()),
        (
            amount,
            recipient.clone(),
            withdrawn_by.clone(),
            new_balance,
            env.ledger().sequence(),
        ),
    );
}

/// Emitted when a fee is collected into the treasury (called by other contracts).
///
/// # Topics: `(fp_treasury, fee_coll, token)`
/// # Data: `(amount: i128, from_contract: Address, new_balance: i128, ledger: u32)`
pub fn emit_fee_collected(
    env: &Env,
    token: &Address,
    amount: i128,
    from_contract: &Address,
    new_balance: i128,
) {
    env.events().publish(
        (topic(env), symbol_short!("fee_coll"), token.clone()),
        (
            amount,
            from_contract.clone(),
            new_balance,
            env.ledger().sequence(),
        ),
    );
}

/// Emitted when a distribution (batch withdrawal) is executed.
///
/// # Topics: `(fp_treasury, distrib, token)`
/// # Data: `(total_amount: i128, recipient_count: u32, authorised_by: Address, ledger: u32)`
pub fn emit_distribution(
    env: &Env,
    token: &Address,
    total_amount: i128,
    recipient_count: u32,
    authorised_by: &Address,
) {
    env.events().publish(
        (topic(env), symbol_short!("distrib"), token.clone()),
        (
            total_amount,
            recipient_count,
            authorised_by.clone(),
            env.ledger().sequence(),
        ),
    );
}
