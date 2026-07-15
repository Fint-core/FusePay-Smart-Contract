//! Ledger storage helpers for the treasury contract.

use crate::types::{TokenLedger, TreasuryConfig};
use shared_constants::{INSTANCE_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS};
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum StorageKey {
    Initialized,
    Config,
    /// Per-token accounting ledger.
    TokenLedger(Address),
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

pub fn get_config(env: &Env) -> Option<TreasuryConfig> {
    env.storage()
        .instance()
        .get::<StorageKey, TreasuryConfig>(&StorageKey::Config)
}

pub fn set_config(env: &Env, config: &TreasuryConfig) {
    env.storage().instance().set(&StorageKey::Config, config);
    bump_instance(env);
}

// ── Token ledger ───────────────────────────────────────────────────────────

pub fn get_token_ledger(env: &Env, token: &Address) -> Option<TokenLedger> {
    let key = StorageKey::TokenLedger(token.clone());
    let result = env
        .storage()
        .persistent()
        .get::<StorageKey, TokenLedger>(&key);
    if result.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
    }
    result
}

pub fn save_token_ledger(env: &Env, ledger: &TokenLedger) {
    let key = StorageKey::TokenLedger(ledger.token.clone());
    env.storage().persistent().set(&key, ledger);
    env.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
}

pub fn get_or_create_token_ledger(env: &Env, token: &Address) -> TokenLedger {
    get_token_ledger(env, token).unwrap_or(TokenLedger {
        token: token.clone(),
        total_deposited: 0,
        total_distributed: 0,
        available: 0,
    })
}

// ── TTL ────────────────────────────────────────────────────────────────────

fn bump_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_BUMP_LEDGERS, INSTANCE_BUMP_LEDGERS);
}
