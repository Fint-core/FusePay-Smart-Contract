//! Access-control contract implementation.
//!
//! This module contains only the `#[contractimpl]` block — thin orchestration
//! that delegates to `storage`, `validation`, `utils`, and `events`.
//! No raw ledger access or business logic lives here.

use crate::errors::AccessControlError;
use crate::events;
use crate::storage;
use crate::types::{AccessConfig, Role, RoleAssignment};
use crate::utils;
use crate::validation;
use soroban_sdk::{contract, contractimpl, Address, Env, Vec};

#[contract]
pub struct AccessControlContract;

#[contractimpl]
impl AccessControlContract {
    // ── Initialisation ─────────────────────────────────────────────────

    /// Initialise the contract with an owner address.
    pub fn initialize(env: Env, owner: Address) -> Result<(), AccessControlError> {
        if storage::is_initialized(&env) {
            return Err(AccessControlError::AlreadyInitialized);
        }

        validation::validate_address(&owner)?;

        // Authenticate: the transaction must be authorised by the owner
        // so that only the intended party can initialise the contract.
        owner.require_auth();

        let assignment = RoleAssignment {
            holder: owner.clone(),
            role: Role::Owner,
            granted_at_ledger: env.ledger().sequence(),
            granted_by: owner.clone(),
        };

        storage::set_owner(&env, &owner);
        storage::grant_role(&env, &assignment);
        storage::set_initialized(&env);

        events::emit_initialized(&env, &owner);
        Ok(())
    }

    // ── Role management ────────────────────────────────────────────────

    /// Grant a role to an address. Caller must be Owner or Admin.
    pub fn grant_role(
        env: Env,
        caller: Address,
        grantee: Address,
        role: Role,
    ) -> Result<(), AccessControlError> {
        caller.require_auth();

        if !storage::is_initialized(&env) {
            return Err(AccessControlError::NotInitialized);
        }

        validation::validate_address(&grantee)?;
        validation::validate_non_owner_role(&role)?;

        // Only Owner or Admin can grant roles.
        utils::require_role(&env, &caller, &Role::Admin)?;

        if storage::has_role(&env, &grantee, &role) {
            return Err(AccessControlError::RoleAlreadyGranted);
        }

        let assignment = RoleAssignment {
            holder: grantee.clone(),
            role: role.clone(),
            granted_at_ledger: env.ledger().sequence(),
            granted_by: caller.clone(),
        };

        storage::grant_role(&env, &assignment);
        events::emit_role_granted(&env, &grantee, &role, &caller);
        Ok(())
    }

    /// Revoke a role from an address. Caller must be Owner or Admin.
    pub fn revoke_role(
        env: Env,
        caller: Address,
        holder: Address,
        role: Role,
    ) -> Result<(), AccessControlError> {
        caller.require_auth();

        if !storage::is_initialized(&env) {
            return Err(AccessControlError::NotInitialized);
        }

        validation::validate_non_owner_role(&role)?;

        utils::require_role(&env, &caller, &Role::Admin)?;

        if !storage::has_role(&env, &holder, &role) {
            return Err(AccessControlError::RoleNotGranted);
        }

        storage::revoke_role(&env, &holder, &role);
        events::emit_role_revoked(&env, &holder, &role, &caller);
        Ok(())
    }

    // ── Ownership transfer ─────────────────────────────────────────────

    /// Transfer ownership to a new address. Only the current Owner can call this.
    pub fn transfer_ownership(
        env: Env,
        caller: Address,
        new_owner: Address,
    ) -> Result<(), AccessControlError> {
        caller.require_auth();

        if !storage::is_initialized(&env) {
            return Err(AccessControlError::NotInitialized);
        }

        validation::validate_address(&new_owner)?;
        utils::require_owner(&env, &caller)?;

        let current_owner = storage::get_owner(&env).ok_or(AccessControlError::NotInitialized)?;
        if new_owner == current_owner {
            return Err(AccessControlError::InvalidOwnerAddress);
        }

        // Revoke Owner role from current owner.
        storage::revoke_role(&env, &current_owner, &Role::Owner);

        // Grant Owner role to new owner.
        let assignment = RoleAssignment {
            holder: new_owner.clone(),
            role: Role::Owner,
            granted_at_ledger: env.ledger().sequence(),
            granted_by: caller.clone(),
        };
        storage::grant_role(&env, &assignment);
        storage::set_owner(&env, &new_owner);

        events::emit_ownership_transferred(&env, &current_owner, &new_owner);
        Ok(())
    }

    // ── View functions ─────────────────────────────────────────────────

    /// Check if an address holds a specific role.
    pub fn has_role(env: Env, address: Address, role: Role) -> bool {
        storage::has_role(&env, &address, &role)
    }

    /// Retrieve the full role assignment record.
    pub fn get_role_assignment(
        env: Env,
        address: Address,
        role: Role,
    ) -> Option<RoleAssignment> {
        storage::get_role_assignment(&env, &address, &role)
    }

    /// List all addresses that hold a given role.
    pub fn get_role_holders(env: Env, role: Role) -> Vec<Address> {
        storage::get_role_index(&env, &role)
    }

    /// Return the current Owner address.
    pub fn get_owner(env: Env) -> Option<Address> {
        storage::get_owner(&env)
    }

    /// Return a full configuration snapshot.
    pub fn get_config(env: Env) -> Result<AccessConfig, AccessControlError> {
        if !storage::is_initialized(&env) {
            return Err(AccessControlError::NotInitialized);
        }

        let owner = storage::get_owner(&env).ok_or(AccessControlError::NotInitialized)?;
        let roles = [
            Role::Owner,
            Role::Admin,
            Role::Operator,
            Role::Treasury,
            Role::Auditor,
        ];

        let mut assignments = Vec::new(&env);
        for role in roles.iter() {
            let holders = storage::get_role_index(&env, role);
            for holder in holders.iter() {
                if let Some(a) = storage::get_role_assignment(&env, &holder, role) {
                    assignments.push_back(a);
                }
            }
        }

        Ok(AccessConfig { owner, assignments })
    }
}
