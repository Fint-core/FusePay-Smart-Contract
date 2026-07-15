//! Ledger storage helpers for the fee-manager contract.

use crate::types::{DiscountRule, FeeSchedule};
use shared_constants::{INSTANCE_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS};
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum StorageKey {
    Initialized,
    Admin,
    FeeSchedule,
    /// Per-address discount rule.
    Discount(Address),
}

// ── Initialisation ─────────────────────────────────────────────────────────

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

// ── Admin ──────────────────────────────────────────────────────────────────

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage()
        .instance()
        .get::<StorageKey, Address>(&StorageKey::Admin)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&StorageKey::Admin, admin);
    bump_instance(env);
}

// ── Fee schedule ───────────────────────────────────────────────────────────

pub fn get_fee_schedule(env: &Env) -> Option<FeeSchedule> {
    env.storage()
        .instance()
        .get::<StorageKey, FeeSchedule>(&StorageKey::FeeSchedule)
}

pub fn set_fee_schedule(env: &Env, schedule: &FeeSchedule) {
    env.storage()
        .instance()
        .set(&StorageKey::FeeSchedule, schedule);
    bump_instance(env);
}

// ── Discount rules ─────────────────────────────────────────────────────────

pub fn get_discount(env: &Env, address: &Address) -> Option<DiscountRule> {
    let key = StorageKey::Discount(address.clone());
    let result = env
        .storage()
        .persistent()
        .get::<StorageKey, DiscountRule>(&key);
    if result.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
    }
    result
}

pub fn set_discount(env: &Env, rule: &DiscountRule) {
    let key = StorageKey::Discount(rule.address.clone());
    env.storage().persistent().set(&key, rule);
    env.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
}

pub fn remove_discount(env: &Env, address: &Address) {
    env.storage()
        .persistent()
        .remove(&StorageKey::Discount(address.clone()));
}

pub fn has_discount(env: &Env, address: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&StorageKey::Discount(address.clone()))
}

// ── TTL ────────────────────────────────────────────────────────────────────

fn bump_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_BUMP_LEDGERS, INSTANCE_BUMP_LEDGERS);
}
