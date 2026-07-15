//! Public interface trait for the access-control contract.
//!
//! Implementing this trait against the contract struct ensures the public API
//! is documented in one place and that the `#[contractimpl]` block is complete.

use crate::errors::AccessControlError;
use crate::types::{AccessConfig, Role, RoleAssignment};
use soroban_sdk::{Address, Env, Vec};

pub trait AccessControlInterface {
    /// Initialise the contract with an initial owner.
    ///
    /// Can only be called once. Subsequent calls return `AlreadyInitialized`.
    fn initialize(env: Env, owner: Address) -> Result<(), AccessControlError>;

    /// Grant a role to an address.
    ///
    /// The caller must be the Owner or an Admin.
    /// Granting the `Owner` role is not allowed; use `transfer_ownership`.
    fn grant_role(
        env: Env,
        caller: Address,
        grantee: Address,
        role: Role,
    ) -> Result<(), AccessControlError>;

    /// Revoke a role from an address.
    ///
    /// The caller must be the Owner or an Admin.
    /// Revoking the `Owner` role is not allowed; use `transfer_ownership`.
    fn revoke_role(
        env: Env,
        caller: Address,
        holder: Address,
        role: Role,
    ) -> Result<(), AccessControlError>;

    /// Returns `true` if `address` currently holds `role`.
    fn has_role(env: Env, address: Address, role: Role) -> bool;

    /// Returns the full `RoleAssignment` record for `address` + `role`, if it exists.
    fn get_role_assignment(
        env: Env,
        address: Address,
        role: Role,
    ) -> Option<RoleAssignment>;

    /// Returns all addresses that hold the given role.
    fn get_role_holders(env: Env, role: Role) -> Vec<Address>;

    /// Transfer ownership to a new address.
    ///
    /// The caller must be the current Owner.
    /// The `new_owner` must not be the same as the current owner.
    fn transfer_ownership(
        env: Env,
        caller: Address,
        new_owner: Address,
    ) -> Result<(), AccessControlError>;

    /// Returns the current Owner address.
    fn get_owner(env: Env) -> Option<Address>;

    /// Returns a full snapshot of the access-control configuration.
    fn get_config(env: Env) -> Result<AccessConfig, AccessControlError>;
}
