//! Storage helpers for the oracle contract.

use crate::types::{OracleConfig, PriceFeed};
use shared_constants::{INSTANCE_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS};
use soroban_sdk::{contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone)]
pub enum StorageKey {
    Initialized,
    Config,
    /// Authorised price provider flag.
    Provider(Address),
    /// Price feed keyed by feed_id string.
    Feed(String),
}

// ── Init ───────────────────────────────────────────────────────────────────

pub fn is_initialized(env: &Env) -> bool {
    env.storage()
        .instance()
        .get::<StorageKey, bool>(&StorageKey::Initialized)
        .unwrap_or(false)
}

pub fn set_initialized(env: &Env) {
    env.storage()
        .instance()
        .set(&StorageKey::Initialized, &true);
    bump_instance(env);
}

// ── Config ─────────────────────────────────────────────────────────────────

pub fn get_config(env: &Env) -> Option<OracleConfig> {
    env.storage()
        .instance()
        .get::<StorageKey, OracleConfig>(&StorageKey::Config)
}

pub fn set_config(env: &Env, config: &OracleConfig) {
    env.storage().instance().set(&StorageKey::Config, config);
    bump_instance(env);
}

// ── Providers ──────────────────────────────────────────────────────────────

pub fn is_provider(env: &Env, address: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&StorageKey::Provider(address.clone()))
}

pub fn add_provider(env: &Env, address: &Address) {
    let key = StorageKey::Provider(address.clone());
    env.storage().persistent().set(&key, &true);
    env.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
}

pub fn remove_provider(env: &Env, address: &Address) {
    env.storage()
        .persistent()
        .remove(&StorageKey::Provider(address.clone()));
}

// ── Price feeds ────────────────────────────────────────────────────────────

pub fn get_feed(env: &Env, feed_id: &String) -> Option<PriceFeed> {
    let key = StorageKey::Feed(feed_id.clone());
    let result = env
        .storage()
        .persistent()
        .get::<StorageKey, PriceFeed>(&key);
    if result.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
    }
    result
}

pub fn save_feed(env: &Env, feed: &PriceFeed) {
    let key = StorageKey::Feed(feed.feed_id.clone());
    env.storage().persistent().set(&key, feed);
    env.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
}

pub fn feed_exists(env: &Env, feed_id: &String) -> bool {
    env.storage()
        .persistent()
        .has(&StorageKey::Feed(feed_id.clone()))
}

// ── TTL ────────────────────────────────────────────────────────────────────

fn bump_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_BUMP_LEDGERS, INSTANCE_BUMP_LEDGERS);
}
