//! Event emission for the access-control contract.
//!
//! Every state-changing operation emits a structured event so that
//! off-chain indexers can reconstruct the full access-control audit trail.

use crate::types::Role;
use soroban_sdk::{symbol_short, Address, Env};

// ── Helpers ────────────────────────────────────────────────────────────────

fn topic(env: &Env) -> soroban_sdk::Symbol {
    soroban_sdk::Symbol::new(env, "fp_access")
}

// ── Events ─────────────────────────────────────────────────────────────────

/// Emitted when the contract is initialised.
///
/// # Topics: `(fp_access, initialized, owner)`
/// # Data: `(ledger: u32)`
pub fn emit_initialized(env: &Env, owner: &Address) {
    env.events().publish(
        (topic(env), symbol_short!("init"), owner.clone()),
        (env.ledger().sequence(),),
    );
}

/// Emitted when a role is granted to an address.
///
/// # Topics: `(fp_access, role_granted, grantee)`
/// # Data: `(role: Role, granted_by: Address, ledger: u32)`
pub fn emit_role_granted(env: &Env, grantee: &Address, role: &Role, granted_by: &Address) {
    env.events().publish(
        (topic(env), symbol_short!("role_grt"), grantee.clone()),
        (role.clone(), granted_by.clone(), env.ledger().sequence()),
    );
}

/// Emitted when a role is revoked from an address.
///
/// # Topics: `(fp_access, role_revoked, holder)`
/// # Data: `(role: Role, revoked_by: Address, ledger: u32)`
pub fn emit_role_revoked(env: &Env, holder: &Address, role: &Role, revoked_by: &Address) {
    env.events().publish(
        (topic(env), symbol_short!("role_rev"), holder.clone()),
        (role.clone(), revoked_by.clone(), env.ledger().sequence()),
    );
}

/// Emitted when contract ownership is transferred.
///
/// # Topics: `(fp_access, owner_xfer, prev_owner)`
/// # Data: `(new_owner: Address, ledger: u32)`
pub fn emit_ownership_transferred(env: &Env, previous_owner: &Address, new_owner: &Address) {
    env.events().publish(
        (
            topic(env),
            symbol_short!("ownr_xfer"),
            previous_owner.clone(),
        ),
        (new_owner.clone(), env.ledger().sequence()),
    );
}
