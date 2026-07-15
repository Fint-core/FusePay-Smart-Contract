//! Ledger storage helpers for the escrow contract.

use crate::types::{Escrow, EscrowConfig};
use shared_constants::{INSTANCE_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS};
use soroban_sdk::{contracttype, Env};

// ── Storage keys ───────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub enum StorageKey {
    Initialized,
    Config,
    /// Next escrow ID counter.
    NextEscrowId,
    /// Escrow record keyed by ID.
    Escrow(u64),
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

// ── Config ─────────────────────────────────────────────────────────────────

pub fn get_config(env: &Env) -> Option<EscrowConfig> {
    env.storage()
        .instance()
        .get::<StorageKey, EscrowConfig>(&StorageKey::Config)
}

pub fn set_config(env: &Env, config: &EscrowConfig) {
    env.storage().instance().set(&StorageKey::Config, config);
    bump_instance(env);
}

// ── Escrow ID counter ──────────────────────────────────────────────────────

pub fn next_escrow_id(env: &Env) -> u64 {
    let current: u64 = env
        .storage()
        .instance()
        .get::<StorageKey, u64>(&StorageKey::NextEscrowId)
        .unwrap_or(0);
    let next = current + 1;
    env.storage()
        .instance()
        .set(&StorageKey::NextEscrowId, &next);
    bump_instance(env);
    next
}

// ── Escrow records ─────────────────────────────────────────────────────────

pub fn get_escrow(env: &Env, escrow_id: u64) -> Option<Escrow> {
    let key = StorageKey::Escrow(escrow_id);
    let result = env
        .storage()
        .persistent()
        .get::<StorageKey, Escrow>(&key);
    if result.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
    }
    result
}

pub fn save_escrow(env: &Env, escrow: &Escrow) {
    let key = StorageKey::Escrow(escrow.escrow_id);
    env.storage().persistent().set(&key, escrow);
    env.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
}

// ── TTL helpers ────────────────────────────────────────────────────────────

fn bump_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_BUMP_LEDGERS, INSTANCE_BUMP_LEDGERS);
}
