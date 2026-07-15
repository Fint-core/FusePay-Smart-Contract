//! Ledger storage helpers for the access-control contract.
//!
//! All storage reads and writes are centralised here.
//! No business logic lives in this module.

use crate::types::{Role, RoleAssignment};
use shared_constants::{INSTANCE_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS};
use soroban_sdk::{contracttype, Address, Env, Vec};

// ── Storage keys ───────────────────────────────────────────────────────────

/// Discriminated union of all storage keys used by this contract.
#[contracttype]
#[derive(Clone)]
pub enum StorageKey {
    /// Marks whether the contract has been initialised.
    Initialized,
    /// The contract owner's address.
    Owner,
    /// Role assignments for a specific address + role pair.
    RoleHolder(Address, Role),
    /// Index: list of all addresses that hold a given role.
    RoleIndex(Role),
}

// ── Initialisation flag ────────────────────────────────────────────────────

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

// ── Owner ──────────────────────────────────────────────────────────────────

pub fn get_owner(env: &Env) -> Option<Address> {
    env.storage()
        .instance()
        .get::<StorageKey, Address>(&StorageKey::Owner)
}

pub fn set_owner(env: &Env, owner: &Address) {
    env.storage()
        .instance()
        .set(&StorageKey::Owner, owner);
    bump_instance(env);
}

// ── Role assignments ───────────────────────────────────────────────────────

/// Returns true if the given address currently holds the given role.
pub fn has_role(env: &Env, address: &Address, role: &Role) -> bool {
    env.storage()
        .persistent()
        .has(&StorageKey::RoleHolder(address.clone(), role.clone()))
}

/// Fetch the full `RoleAssignment` record for an address+role pair.
pub fn get_role_assignment(
    env: &Env,
    address: &Address,
    role: &Role,
) -> Option<RoleAssignment> {
    let key = StorageKey::RoleHolder(address.clone(), role.clone());
    let result = env
        .storage()
        .persistent()
        .get::<StorageKey, RoleAssignment>(&key);
    if result.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);
    }
    result
}

/// Store a role assignment and update the role index.
pub fn grant_role(env: &Env, assignment: &RoleAssignment) {
    let key = StorageKey::RoleHolder(assignment.holder.clone(), assignment.role.clone());
    env.storage()
        .persistent()
        .set(&key, assignment);
    env.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_BUMP_LEDGERS, PERSISTENT_BUMP_LEDGERS);

    // Update role index
    let mut index = get_role_index(env, &assignment.role);
    if !index.contains(&assignment.holder) {
        index.push_back(assignment.holder.clone());
        set_role_index(env, &assignment.role, &index);
    }
}

/// Remove a role assignment and update the role index.
pub fn revoke_role(env: &Env, address: &Address, role: &Role) {
    let key = StorageKey::RoleHolder(address.clone(), role.clone());
    env.storage().persistent().remove(&key);

    // Update role index — rebuild without the revoked address.
    let index = get_role_index(env, role);
    let mut new_index: Vec<Address> = Vec::new(env);
    for a in index.iter() {
        if &a != address {
            new_index.push_back(a);
        }
    }
    set_role_index(env, role, &new_index);
}

// ── Role index ─────────────────────────────────────────────────────────────

pub fn get_role_index(env: &Env, role: &Role) -> Vec<Address> {
    let key = StorageKey::RoleIndex(role.clone());
    env.storage()
        .persistent()
        .get::<StorageKey, Vec<Address>>(&key)
        .unwrap_or_else(|| Vec::new(env))
}

fn set_role_index(env: &Env, role: &Role, index: &Vec<Address>) {
    let key = StorageKey::RoleIndex(role.clone());
    env.storage().persistent().set(&key, index);
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
