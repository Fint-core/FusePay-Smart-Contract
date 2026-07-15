//! Pure utility functions for the access-control contract.

use crate::errors::AccessControlError;
use crate::storage;
use crate::types::Role;
use soroban_sdk::{Address, Env};

/// Assert that `caller` holds at least the given `required_role`.
///
/// Role hierarchy (highest → lowest): Owner > Admin > Operator / Treasury > Auditor.
/// A caller with a higher role satisfies a check for a lower role.
///
/// Returns `AccessControlError::Unauthorized` if the check fails.
pub fn require_role(
    env: &Env,
    caller: &Address,
    required_role: &Role,
) -> Result<(), AccessControlError> {
    // Owner satisfies all role checks.
    if storage::has_role(env, caller, &Role::Owner) {
        return Ok(());
    }
    // Admin satisfies Admin, Operator, Treasury, and Auditor checks.
    if *required_role != Role::Owner && storage::has_role(env, caller, &Role::Admin) {
        return Ok(());
    }
    // Direct role check.
    if storage::has_role(env, caller, required_role) {
        return Ok(());
    }
    Err(AccessControlError::Unauthorized)
}

/// Assert that `caller` is the current Owner.
///
/// This is a stricter check that does not accept other roles.
pub fn require_owner(env: &Env, caller: &Address) -> Result<(), AccessControlError> {
    let owner = storage::get_owner(env).ok_or(AccessControlError::NotInitialized)?;
    if &owner != caller {
        return Err(AccessControlError::Unauthorized);
    }
    Ok(())
}
