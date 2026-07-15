//! Unit tests for the access-control contract.

#![cfg(test)]

use fusepay_access_control::{AccessControlContract, AccessControlContractClient, AccessControlError, Role};
use soroban_sdk::{testutils::Address as _, Address, Env};

// ── Test helpers ───────────────────────────────────────────────────────────

fn setup() -> (Env, AccessControlContractClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AccessControlContract);
    let client = AccessControlContractClient::new(&env, &contract_id);
    let owner = Address::generate(&env);

    (env, client, owner)
}

// ── Initialisation ─────────────────────────────────────────────────────────

#[test]
fn test_initialize_sets_owner() {
    let (env, client, owner) = setup();
    client.initialize(&owner);
    assert_eq!(client.get_owner(), Some(owner));
}

#[test]
fn test_initialize_grants_owner_role() {
    let (env, client, owner) = setup();
    client.initialize(&owner);
    assert!(client.has_role(&owner, &Role::Owner));
}

#[test]
fn test_initialize_cannot_be_called_twice() {
    let (env, client, owner) = setup();
    client.initialize(&owner);
    let result = client.try_initialize(&owner);
    assert_eq!(result, Err(Ok(AccessControlError::AlreadyInitialized)));
}

// ── Role grant ─────────────────────────────────────────────────────────────

#[test]
fn test_owner_can_grant_admin_role() {
    let (env, client, owner) = setup();
    client.initialize(&owner);

    let admin = Address::generate(&env);
    client.grant_role(&owner, &admin, &Role::Admin);
    assert!(client.has_role(&admin, &Role::Admin));
}

#[test]
fn test_admin_can_grant_operator_role() {
    let (env, client, owner) = setup();
    client.initialize(&owner);

    let admin = Address::generate(&env);
    client.grant_role(&owner, &admin, &Role::Admin);

    let operator = Address::generate(&env);
    client.grant_role(&admin, &operator, &Role::Operator);
    assert!(client.has_role(&operator, &Role::Operator));
}

#[test]
fn test_cannot_grant_owner_role_directly() {
    let (env, client, owner) = setup();
    client.initialize(&owner);

    let new_addr = Address::generate(&env);
    let result = client.try_grant_role(&owner, &new_addr, &Role::Owner);
    assert_eq!(result, Err(Ok(AccessControlError::CannotRevokeOwner)));
}

#[test]
fn test_cannot_grant_already_held_role() {
    let (env, client, owner) = setup();
    client.initialize(&owner);

    let admin = Address::generate(&env);
    client.grant_role(&owner, &admin, &Role::Admin);
    let result = client.try_grant_role(&owner, &admin, &Role::Admin);
    assert_eq!(result, Err(Ok(AccessControlError::RoleAlreadyGranted)));
}

#[test]
fn test_unauthorized_cannot_grant_role() {
    let (env, client, owner) = setup();
    client.initialize(&owner);

    let random = Address::generate(&env);
    let target = Address::generate(&env);
    let result = client.try_grant_role(&random, &target, &Role::Operator);
    assert_eq!(result, Err(Ok(AccessControlError::Unauthorized)));
}

// ── Role revoke ────────────────────────────────────────────────────────────

#[test]
fn test_owner_can_revoke_role() {
    let (env, client, owner) = setup();
    client.initialize(&owner);

    let admin = Address::generate(&env);
    client.grant_role(&owner, &admin, &Role::Admin);
    client.revoke_role(&owner, &admin, &Role::Admin);
    assert!(!client.has_role(&admin, &Role::Admin));
}

#[test]
fn test_revoke_non_held_role_fails() {
    let (env, client, owner) = setup();
    client.initialize(&owner);

    let addr = Address::generate(&env);
    let result = client.try_revoke_role(&owner, &addr, &Role::Operator);
    assert_eq!(result, Err(Ok(AccessControlError::RoleNotGranted)));
}

// ── Ownership transfer ─────────────────────────────────────────────────────

#[test]
fn test_owner_can_transfer_ownership() {
    let (env, client, owner) = setup();
    client.initialize(&owner);

    let new_owner = Address::generate(&env);
    client.transfer_ownership(&owner, &new_owner);

    assert_eq!(client.get_owner(), Some(new_owner.clone()));
    assert!(client.has_role(&new_owner, &Role::Owner));
    assert!(!client.has_role(&owner, &Role::Owner));
}

#[test]
fn test_non_owner_cannot_transfer_ownership() {
    let (env, client, owner) = setup();
    client.initialize(&owner);

    let random = Address::generate(&env);
    let new_owner = Address::generate(&env);
    let result = client.try_transfer_ownership(&random, &new_owner);
    assert_eq!(result, Err(Ok(AccessControlError::Unauthorized)));
}

#[test]
fn test_cannot_transfer_to_same_owner() {
    let (env, client, owner) = setup();
    client.initialize(&owner);

    let result = client.try_transfer_ownership(&owner, &owner);
    assert_eq!(result, Err(Ok(AccessControlError::InvalidOwnerAddress)));
}

// ── View functions ─────────────────────────────────────────────────────────

#[test]
fn test_get_role_holders_returns_all() {
    let (env, client, owner) = setup();
    client.initialize(&owner);

    let op1 = Address::generate(&env);
    let op2 = Address::generate(&env);
    client.grant_role(&owner, &op1, &Role::Operator);
    client.grant_role(&owner, &op2, &Role::Operator);

    let holders = client.get_role_holders(&Role::Operator);
    assert_eq!(holders.len(), 2);
}

#[test]
fn test_get_config_returns_full_snapshot() {
    let (env, client, owner) = setup();
    client.initialize(&owner);

    let admin = Address::generate(&env);
    client.grant_role(&owner, &admin, &Role::Admin);

    let config = client.get_config();
    assert_eq!(config.owner, owner);
    // Owner + Admin assignments
    assert_eq!(config.assignments.len(), 2);
}

// ── Security tests ─────────────────────────────────────────────────────────

#[test]
fn test_auditor_cannot_grant_roles() {
    let (env, client, owner) = setup();
    client.initialize(&owner);

    let auditor = Address::generate(&env);
    client.grant_role(&owner, &auditor, &Role::Auditor);

    let target = Address::generate(&env);
    let result = client.try_grant_role(&auditor, &target, &Role::Operator);
    assert_eq!(result, Err(Ok(AccessControlError::Unauthorized)));
}

#[test]
fn test_operator_cannot_grant_roles() {
    let (env, client, owner) = setup();
    client.initialize(&owner);

    let operator = Address::generate(&env);
    client.grant_role(&owner, &operator, &Role::Operator);

    let target = Address::generate(&env);
    let result = client.try_grant_role(&operator, &target, &Role::Auditor);
    assert_eq!(result, Err(Ok(AccessControlError::Unauthorized)));
}

#[test]
fn test_view_functions_before_init_return_none() {
    let (_env, client, _owner) = setup();
    // get_owner should return None before init
    assert_eq!(client.get_owner(), None);
}
