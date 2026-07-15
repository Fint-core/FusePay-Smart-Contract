//! Event emission for the wallet contract.

use soroban_sdk::{symbol_short, Address, Env, Symbol};

fn topic(env: &Env) -> Symbol {
    Symbol::new(env, "fp_wallet")
}

/// Emitted when a new wallet is created.
///
/// # Topics: `(fp_wallet, created, owner)`
/// # Data: `(token: Address, ledger: u32)`
pub fn emit_wallet_created(env: &Env, owner: &Address, token: &Address) {
    env.events().publish(
        (topic(env), symbol_short!("created"), owner.clone()),
        (token.clone(), env.ledger().sequence()),
    );
}

/// Emitted on a successful deposit.
///
/// # Topics: `(fp_wallet, deposit, owner)`
/// # Data: `(token: Address, amount: i128, new_balance: i128, ledger: u32)`
pub fn emit_deposit(env: &Env, owner: &Address, token: &Address, amount: i128, new_balance: i128) {
    env.events().publish(
        (topic(env), symbol_short!("deposit"), owner.clone()),
        (token.clone(), amount, new_balance, env.ledger().sequence()),
    );
}

/// Emitted on a successful withdrawal.
///
/// # Topics: `(fp_wallet, withdraw, owner)`
/// # Data: `(token: Address, amount: i128, recipient: Address, new_balance: i128, ledger: u32)`
pub fn emit_withdrawal(
    env: &Env,
    owner: &Address,
    token: &Address,
    amount: i128,
    recipient: &Address,
    new_balance: i128,
) {
    env.events().publish(
        (topic(env), symbol_short!("withdraw"), owner.clone()),
        (
            token.clone(),
            amount,
            recipient.clone(),
            new_balance,
            env.ledger().sequence(),
        ),
    );
}

/// Emitted when a wallet is frozen by an admin.
///
/// # Topics: `(fp_wallet, frozen, owner)`
/// # Data: `(frozen_by: Address, ledger: u32)`
pub fn emit_wallet_frozen(env: &Env, owner: &Address, frozen_by: &Address) {
    env.events().publish(
        (topic(env), symbol_short!("frozen"), owner.clone()),
        (frozen_by.clone(), env.ledger().sequence()),
    );
}

/// Emitted when a wallet is unfrozen by an admin.
///
/// # Topics: `(fp_wallet, unfrozen, owner)`
/// # Data: `(unfrozen_by: Address, ledger: u32)`
pub fn emit_wallet_unfrozen(env: &Env, owner: &Address, unfrozen_by: &Address) {
    env.events().publish(
        (topic(env), symbol_short!("unfrozn"), owner.clone()),
        (unfrozen_by.clone(), env.ledger().sequence()),
    );
}
