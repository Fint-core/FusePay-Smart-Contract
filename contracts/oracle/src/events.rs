//! Event emission for the oracle contract.

use soroban_sdk::{symbol_short, Address, Env, String, Symbol};

fn topic(env: &Env) -> Symbol {
    Symbol::new(env, "fp_oracle")
}

/// Emitted when a price feed is updated.
pub fn emit_price_updated(
    env: &Env,
    feed_id: &String,
    provider: &Address,
    price: i128,
) {
    env.events().publish(
        (topic(env), symbol_short!("price_upd"), feed_id.clone()),
        (price, provider.clone(), env.ledger().sequence()),
    );
}

/// Emitted when a new feed is registered.
pub fn emit_feed_registered(env: &Env, feed_id: &String, admin: &Address) {
    env.events().publish(
        (topic(env), symbol_short!("feed_reg"), feed_id.clone()),
        (admin.clone(), env.ledger().sequence()),
    );
}

/// Emitted when a provider is added.
pub fn emit_provider_added(env: &Env, provider: &Address, admin: &Address) {
    env.events().publish(
        (topic(env), symbol_short!("prov_add"), provider.clone()),
        (admin.clone(), env.ledger().sequence()),
    );
}

/// Emitted when a provider is removed.
pub fn emit_provider_removed(env: &Env, provider: &Address, admin: &Address) {
    env.events().publish(
        (topic(env), symbol_short!("prov_rem"), provider.clone()),
        (admin.clone(), env.ledger().sequence()),
    );
}
