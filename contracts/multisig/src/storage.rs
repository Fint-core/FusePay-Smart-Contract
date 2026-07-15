//! Ledger storage helpers for the multisig contract.

use crate::types::{MultisigConfig, Proposal, Vote};
use shared_constants::{INSTANCE_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS};
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum StorageKey {
    Initialized,
    Config,
    NextProposalId,
    Proposal(u64),
    /// Tracks whether owner `Address` has voted on proposal `u64`.
    Vote(u64, Address),
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

pub fn get_config(env: &Env) -> Option<MultisigConfig> {
    env.storage()
        .instance()
        .get::<StorageKey, MultisigConfig>(&StorageKey::Config)
}

pub fn set_config(env: &Env, config: &MultisigConfig) {
    env.storage().instance().set(&StorageKey::Config, config);
    bump_instance(env);
}

// ── Proposal ID counter ────────────────────────────────────────────────────

pub fn next_proposal_id(env: &Env) -> u64 {
    let current: u64 = env
        .storage()
        .instance()
        .get::<StorageKey, u64>(&StorageKey::NextProposalId)
        .unwrap_or(0);
    let next = current + 1;
    env.storage()
        .instance()
        .set(&StorageKey::NextProposalId, &next);
    bump_instance(env);
    next
}

// ── Proposals ──────────────────────────────────────────────────────────────

pub fn get_proposal(env: &Env, id: u64) -> Option<Proposal> {
    let key = StorageKey::Proposal(id);
    let result = env
        .storage()
        .persistent()
        .get::<StorageKey, Proposal>(&key);
    if result.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
    }
    result
}

pub fn save_proposal(env: &Env, proposal: &Proposal) {
    let key = StorageKey::Proposal(proposal.proposal_id);
    env.storage().persistent().set(&key, proposal);
    env.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
}

// ── Votes ──────────────────────────────────────────────────────────────────

pub fn get_vote(env: &Env, proposal_id: u64, owner: &Address) -> Option<Vote> {
    env.storage()
        .persistent()
        .get::<StorageKey, Vote>(&StorageKey::Vote(proposal_id, owner.clone()))
}

pub fn save_vote(env: &Env, proposal_id: u64, owner: &Address, vote: &Vote) {
    let key = StorageKey::Vote(proposal_id, owner.clone());
    env.storage().persistent().set(&key, vote);
    env.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
}

pub fn has_voted(env: &Env, proposal_id: u64, owner: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&StorageKey::Vote(proposal_id, owner.clone()))
}

// ── TTL ────────────────────────────────────────────────────────────────────

fn bump_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_BUMP_LEDGERS, INSTANCE_BUMP_LEDGERS);
}
