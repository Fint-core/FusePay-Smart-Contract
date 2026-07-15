//! Input validation for the access-control contract.

use crate::errors::AccessControlError;
use crate::types::Role;
use soroban_sdk::Address;

/// Validate that an address can be used as a role holder.
///
/// Soroban's `Address` type guarantees the bytes represent a valid
/// Stellar account or contract. This function is a hook for future
/// stricter validation (e.g. blocklists) without changing call sites.
pub fn validate_address(_address: &Address) -> Result<(), AccessControlError> {
    Ok(())
}

/// Ensure the Owner role is not passed to `grant_role` / `revoke_role`.
/// Ownership transfers must go through `transfer_ownership`.
pub fn validate_non_owner_role(role: &Role) -> Result<(), AccessControlError> {
    if *role == Role::Owner {
        return Err(AccessControlError::CannotRevokeOwner);
    }
    Ok(())
}
