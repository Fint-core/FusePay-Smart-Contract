//! Unit tests for the multisig contract.

#![cfg(test)]

use fusepay_multisig::{
    MultisigContract, MultisigContractClient, MultisigError, ProposalState,
};
use soroban_sdk::{testutils::{Address as _, Ledger as _}, Address, Bytes, Env, String, Vec};

// ── Helpers ────────────────────────────────────────────────────────────────

fn make_owners(env: &Env, n: usize) -> Vec<Address> {
    let mut v = Vec::new(env);
    for _ in 0..n {
        v.push_back(Address::generate(env));
    }
    v
}

fn setup_2of3(env: &Env) -> (MultisigContractClient, Vec<Address>) {
    let contract_id = env.register_contract(None, MultisigContract);
    let client = MultisigContractClient::new(env, &contract_id);
    let owners = make_owners(env, 3);
    client.initialize(&owners, &2);
    (client, owners)
}

fn empty_call_data(env: &Env) -> Bytes {
    Bytes::new(env)
}

fn dummy_target(env: &Env) -> Address {
    Address::generate(env)
}

// ── Initialisation ─────────────────────────────────────────────────────────

#[test]
fn test_initialize_stores_config() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, owners) = setup_2of3(&env);

    let config = client.get_config();
    assert_eq!(config.threshold, 2);
    assert_eq!(config.owners.len(), 3);
    // Verify all owners are present
    for owner in owners.iter() {
        assert!(config.owners.contains(&owner));
    }
}

#[test]
fn test_double_init_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, owners) = setup_2of3(&env);
    let result = client.try_initialize(&owners, &2);
    assert_eq!(result, Err(Ok(MultisigError::AlreadyInitialized)));
}

#[test]
fn test_threshold_exceeds_owner_count_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, MultisigContract);
    let client = MultisigContractClient::new(&env, &contract_id);
    let owners = make_owners(&env, 2);
    let result = client.try_initialize(&owners, &3); // threshold > owners
    assert_eq!(result, Err(Ok(MultisigError::InvalidThreshold)));
}

#[test]
fn test_zero_threshold_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, MultisigContract);
    let client = MultisigContractClient::new(&env, &contract_id);
    let owners = make_owners(&env, 2);
    let result = client.try_initialize(&owners, &0);
    assert_eq!(result, Err(Ok(MultisigError::InvalidThreshold)));
}

#[test]
fn test_duplicate_owner_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, MultisigContract);
    let client = MultisigContractClient::new(&env, &contract_id);

    let addr = Address::generate(&env);
    let mut owners = Vec::new(&env);
    owners.push_back(addr.clone());
    owners.push_back(addr.clone()); // duplicate

    let result = client.try_initialize(&owners, &1);
    assert_eq!(result, Err(Ok(MultisigError::DuplicateOwner)));
}

// ── Submit proposal ────────────────────────────────────────────────────────

#[test]
fn test_submit_proposal_returns_id() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, owners) = setup_2of3(&env);

    let id = client.submit_proposal(
        &owners.get(0).unwrap(),
        &String::from_str(&env, "test proposal"),
        &dummy_target(&env),
        &empty_call_data(&env),
        &0,
    );

    assert_eq!(id, 1);
}

#[test]
fn test_non_owner_cannot_submit() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = setup_2of3(&env);
    let outsider = Address::generate(&env);

    let result = client.try_submit_proposal(
        &outsider,
        &String::from_str(&env, "bad proposal"),
        &dummy_target(&env),
        &empty_call_data(&env),
        &0,
    );
    assert_eq!(result, Err(Ok(MultisigError::Unauthorized)));
}

// ── Voting ─────────────────────────────────────────────────────────────────

#[test]
fn test_single_approval_does_not_execute() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, owners) = setup_2of3(&env);

    let id = client.submit_proposal(
        &owners.get(0).unwrap(),
        &String::from_str(&env, "p1"),
        &dummy_target(&env),
        &empty_call_data(&env),
        &0,
    );

    client.approve(&owners.get(0).unwrap(), &id);
    let proposal = client.get_proposal(&id);
    assert_eq!(proposal.state, ProposalState::Active);
    assert_eq!(proposal.approval_count, 1);
}

#[test]
fn test_two_approvals_executes_2of3() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, owners) = setup_2of3(&env);

    let id = client.submit_proposal(
        &owners.get(0).unwrap(),
        &String::from_str(&env, "p1"),
        &dummy_target(&env),
        &empty_call_data(&env),
        &0,
    );

    client.approve(&owners.get(0).unwrap(), &id);
    client.approve(&owners.get(1).unwrap(), &id);

    let proposal = client.get_proposal(&id);
    assert_eq!(proposal.state, ProposalState::Executed);
    assert!(client.is_approved(&id));
}

#[test]
fn test_double_vote_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, owners) = setup_2of3(&env);

    let id = client.submit_proposal(
        &owners.get(0).unwrap(),
        &String::from_str(&env, "p1"),
        &dummy_target(&env),
        &empty_call_data(&env),
        &0,
    );

    client.approve(&owners.get(0).unwrap(), &id);
    let result = client.try_approve(&owners.get(0).unwrap(), &id);
    assert_eq!(result, Err(Ok(MultisigError::AlreadyVoted)));
}

#[test]
fn test_blocking_rejections_rejects_proposal() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, owners) = setup_2of3(&env); // threshold = 2, owners = 3

    let id = client.submit_proposal(
        &owners.get(0).unwrap(),
        &String::from_str(&env, "p1"),
        &dummy_target(&env),
        &empty_call_data(&env),
        &0,
    );

    // With 3 owners and threshold 2, 2 rejections make it impossible to reach threshold.
    client.reject(&owners.get(0).unwrap(), &id);
    client.reject(&owners.get(1).unwrap(), &id);

    let proposal = client.get_proposal(&id);
    assert_eq!(proposal.state, ProposalState::Rejected);
}

#[test]
fn test_vote_on_executed_proposal_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, owners) = setup_2of3(&env);

    let id = client.submit_proposal(
        &owners.get(0).unwrap(),
        &String::from_str(&env, "p1"),
        &dummy_target(&env),
        &empty_call_data(&env),
        &0,
    );

    client.approve(&owners.get(0).unwrap(), &id);
    client.approve(&owners.get(1).unwrap(), &id);

    // proposal is now Executed
    let result = client.try_approve(&owners.get(2).unwrap(), &id);
    assert_eq!(result, Err(Ok(MultisigError::InvalidState)));
}

// ── Expiry ─────────────────────────────────────────────────────────────────

#[test]
fn test_vote_on_expired_proposal_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, owners) = setup_2of3(&env);

    let id = client.submit_proposal(
        &owners.get(0).unwrap(),
        &String::from_str(&env, "p1"),
        &dummy_target(&env),
        &empty_call_data(&env),
        &1, // 1 ledger expiry
    );

    // Advance past expiry
    env.ledger().with_mut(|l| {
        l.sequence_number += 100;
    });

    let result = client.try_approve(&owners.get(0).unwrap(), &id);
    assert_eq!(result, Err(Ok(MultisigError::ProposalExpired)));
}

#[test]
fn test_expire_proposal_after_expiry() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, owners) = setup_2of3(&env);

    let id = client.submit_proposal(
        &owners.get(0).unwrap(),
        &String::from_str(&env, "p1"),
        &dummy_target(&env),
        &empty_call_data(&env),
        &1,
    );

    env.ledger().with_mut(|l| {
        l.sequence_number += 100;
    });

    let caller = Address::generate(&env);
    client.expire_proposal(&caller, &id);

    let proposal = client.get_proposal(&id);
    assert_eq!(proposal.state, ProposalState::Expired);
}

// ── Security ───────────────────────────────────────────────────────────────

#[test]
fn test_is_approved_returns_false_for_active() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, owners) = setup_2of3(&env);

    let id = client.submit_proposal(
        &owners.get(0).unwrap(),
        &String::from_str(&env, "p1"),
        &dummy_target(&env),
        &empty_call_data(&env),
        &0,
    );

    assert!(!client.is_approved(&id));
}

#[test]
fn test_is_approved_returns_false_for_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, owners) = setup_2of3(&env);

    let id = client.submit_proposal(
        &owners.get(0).unwrap(),
        &String::from_str(&env, "p1"),
        &dummy_target(&env),
        &empty_call_data(&env),
        &0,
    );

    client.reject(&owners.get(0).unwrap(), &id);
    client.reject(&owners.get(1).unwrap(), &id);

    assert!(!client.is_approved(&id));
}
