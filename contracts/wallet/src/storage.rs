//! Ledger storage helpers for the wallet contract.

use crate::types::Wallet;
use shared_constants::{INSTANCE_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS};
use soroban_sdk::{contracttype, Address, Env};

// ── Storage keys ───────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub enum StorageKey {
    /// Marks whether the contract has been initialised.
    Initialized,
    /// The platform admin address (can freeze/unfreeze wallets).
    Admin,
    /// Wallet record keyed by (owner, token).
    Wallet(Address, Address),
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

// ── Wallet ─────────────────────────────────────────────────────────────────

pub fn get_wallet(env: &Env, owner: &Address, token: &Address) -> Option<Wallet> {
    let key = StorageKey::Wallet(owner.clone(), token.clone());
    let result = env
        .storage()
        .persistent()
        .get::<StorageKey, Wallet>(&key);
    if result.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
    }
    result
}

pub fn save_wallet(env: &Env, wallet: &Wallet) {
    let key = StorageKey::Wallet(wallet.owner.clone(), wallet.token.clone());
    env.storage().persistent().set(&key, wallet);
    env.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
}

pub fn wallet_exists(env: &Env, owner: &Address, token: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&StorageKey::Wallet(owner.clone(), token.clone()))
}

// ── TTL helpers ────────────────────────────────────────────────────────────

fn bump_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_BUMP_LEDGERS, INSTANCE_BUMP_LEDGERS);
}
