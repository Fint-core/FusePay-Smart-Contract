//! Multisig contract implementation.
//!
//! ## Execution model
//!
//! When a proposal reaches the approval threshold, it is marked `Executed` and
//! the `target_contract` is invoked via a cross-contract call using the stored
//! `call_data`.  Because Soroban does not have a low-level `call(bytes)` ABI,
//! the actual dispatch is left to an integrating contract (e.g. treasury) that
//! checks `has_approval` before executing privileged actions.  This contract
//! records that the threshold was reached and emits an `Executed` event which
//! off-chain automation and other on-chain guards can rely on.

use crate::errors::MultisigError;
use crate::events;
use crate::storage;
use crate::types::{MultisigConfig, Proposal, ProposalState, Vote};
use crate::utils;
use crate::validation;
use shared_constants::{DEFAULT_MULTISIG_EXPIRY_LEDGERS, MIN_MULTISIG_THRESHOLD};
use shared_utils::expiry_ledger;
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, String, Vec};

#[contract]
pub struct MultisigContract;

#[contractimpl]
impl MultisigContract {
    // ── Initialisation ─────────────────────────────────────────────────

    pub fn initialize(
        env: Env,
        owners: Vec<Address>,
        threshold: u32,
    ) -> Result<(), MultisigError> {
        if storage::is_initialized(&env) {
            return Err(MultisigError::AlreadyInitialized);
        }

        validation::validate_config(&owners, threshold)?;

        // Require auth from every owner to prevent unauthorised configuration.
        for owner in owners.iter() {
            owner.require_auth();
        }

        let config = MultisigConfig { owners, threshold };
        storage::set_config(&env, &config);
        storage::set_initialized(&env);
        Ok(())
    }

    // ── Submit proposal ────────────────────────────────────────────────

    pub fn submit_proposal(
        env: Env,
        proposer: Address,
        description: String,
        target_contract: Address,
        call_data: Bytes,
        expiry_ledgers: u32,
    ) -> Result<u64, MultisigError> {
        if !storage::is_initialized(&env) {
            return Err(MultisigError::NotInitialized);
        }

        proposer.require_auth();

        let config = storage::get_config(&env).ok_or(MultisigError::NotInitialized)?;
        validation::require_owner(&config.owners, &proposer)?;

        // Use default expiry if caller passes 0.
        let timeout = if expiry_ledgers == 0 {
            DEFAULT_MULTISIG_EXPIRY_LEDGERS
        } else {
            validation::validate_expiry(expiry_ledgers)?;
            expiry_ledgers
        };

        let expires_at =
            expiry_ledger(&env, timeout).map_err(|_| MultisigError::ArithmeticOverflow)?;

        let proposal_id = storage::next_proposal_id(&env);
        let proposal = Proposal {
            proposal_id,
            description,
            target_contract,
            call_data,
            approval_count: 0,
            rejection_count: 0,
            state: ProposalState::Active,
            expires_at_ledger: expires_at,
            created_at_ledger: env.ledger().sequence(),
            proposer: proposer.clone(),
        };

        storage::save_proposal(&env, &proposal);
        events::emit_proposal_created(&env, proposal_id, &proposer, expires_at);
        Ok(proposal_id)
    }

    // ── Approve ────────────────────────────────────────────────────────

    pub fn approve(env: Env, owner: Address, proposal_id: u64) -> Result<(), MultisigError> {
        if !storage::is_initialized(&env) {
            return Err(MultisigError::NotInitialized);
        }

        owner.require_auth();

        let config = storage::get_config(&env).ok_or(MultisigError::NotInitialized)?;
        validation::require_owner(&config.owners, &owner)?;

        let mut proposal = storage::get_proposal(&env, proposal_id)
            .ok_or(MultisigError::ProposalNotFound)?;

        // Proposal must be Active to receive votes.
        if proposal.state != ProposalState::Active {
            return Err(MultisigError::InvalidState);
        }

        validation::require_not_expired(&env, proposal.expires_at_ledger)?;

        if storage::has_voted(&env, proposal_id, &owner) {
            return Err(MultisigError::AlreadyVoted);
        }

        proposal.approval_count = utils::increment_counter(proposal.approval_count)?;
        storage::save_vote(&env, proposal_id, &owner, &Vote::Approve);
        events::emit_vote_cast(&env, proposal_id, &owner, &Vote::Approve);

        // Check if threshold is reached.
        if proposal.approval_count >= config.threshold {
            proposal.state = ProposalState::Executed;
            storage::save_proposal(&env, &proposal);
            events::emit_proposal_executed(&env, proposal_id, &owner);
        } else {
            storage::save_proposal(&env, &proposal);
        }

        Ok(())
    }

    // ── Reject ─────────────────────────────────────────────────────────

    pub fn reject(env: Env, owner: Address, proposal_id: u64) -> Result<(), MultisigError> {
        if !storage::is_initialized(&env) {
            return Err(MultisigError::NotInitialized);
        }

        owner.require_auth();

        let config = storage::get_config(&env).ok_or(MultisigError::NotInitialized)?;
        validation::require_owner(&config.owners, &owner)?;

        let mut proposal = storage::get_proposal(&env, proposal_id)
            .ok_or(MultisigError::ProposalNotFound)?;

        if proposal.state != ProposalState::Active {
            return Err(MultisigError::InvalidState);
        }

        validation::require_not_expired(&env, proposal.expires_at_ledger)?;

        if storage::has_voted(&env, proposal_id, &owner) {
            return Err(MultisigError::AlreadyVoted);
        }

        proposal.rejection_count = utils::increment_counter(proposal.rejection_count)?;
        storage::save_vote(&env, proposal_id, &owner, &Vote::Reject);
        events::emit_vote_cast(&env, proposal_id, &owner, &Vote::Reject);

        // A proposal is permanently blocked when remaining possible approvals
        // can no longer reach the threshold.
        // remaining_possible = owners - rejections
        // if remaining_possible < threshold  →  reject the proposal
        let owner_count = config.owners.len();
        let remaining_possible = owner_count
            .checked_sub(proposal.rejection_count)
            .unwrap_or(0);

        if remaining_possible < config.threshold {
            proposal.state = ProposalState::Rejected;
            storage::save_proposal(&env, &proposal);
            events::emit_proposal_rejected(&env, proposal_id);
        } else {
            storage::save_proposal(&env, &proposal);
        }

        Ok(())
    }

    // ── Expire ─────────────────────────────────────────────────────────

    pub fn expire_proposal(
        env: Env,
        caller: Address,
        proposal_id: u64,
    ) -> Result<(), MultisigError> {
        if !storage::is_initialized(&env) {
            return Err(MultisigError::NotInitialized);
        }

        caller.require_auth();

        let mut proposal = storage::get_proposal(&env, proposal_id)
            .ok_or(MultisigError::ProposalNotFound)?;

        if proposal.state != ProposalState::Active {
            return Err(MultisigError::InvalidState);
        }

        // Can only expire if the expiry ledger has passed.
        if env.ledger().sequence() < proposal.expires_at_ledger {
            return Err(MultisigError::ProposalExpired); // not expired yet
        }

        proposal.state = ProposalState::Expired;
        storage::save_proposal(&env, &proposal);
        events::emit_proposal_expired(&env, proposal_id, &caller);
        Ok(())
    }

    // ── View functions ─────────────────────────────────────────────────

    pub fn get_config(env: Env) -> Result<MultisigConfig, MultisigError> {
        if !storage::is_initialized(&env) {
            return Err(MultisigError::NotInitialized);
        }
        storage::get_config(&env).ok_or(MultisigError::NotInitialized)
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Result<Proposal, MultisigError> {
        storage::get_proposal(&env, proposal_id).ok_or(MultisigError::ProposalNotFound)
    }

    /// Returns true if the given proposal has reached the execution threshold.
    /// Used by other contracts (e.g. treasury) to gate privileged operations.
    pub fn is_approved(env: Env, proposal_id: u64) -> bool {
        storage::get_proposal(&env, proposal_id)
            .map(|p| p.state == ProposalState::Executed)
            .unwrap_or(false)
    }
}
