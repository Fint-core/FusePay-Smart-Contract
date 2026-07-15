//! Public interface trait for the multisig contract.

use crate::errors::MultisigError;
use crate::types::{MultisigConfig, Proposal};
use soroban_sdk::{Address, Bytes, Env, String, Vec};

pub trait MultisigInterface {
    /// Initialise the multisig with an owner list and approval threshold.
    fn initialize(
        env: Env,
        owners: Vec<Address>,
        threshold: u32,
    ) -> Result<(), MultisigError>;

    /// Submit a new proposal.
    ///
    /// The proposer must be one of the configured owners.
    /// Returns the new proposal ID.
    fn submit_proposal(
        env: Env,
        proposer: Address,
        description: String,
        target_contract: Address,
        call_data: Bytes,
        expiry_ledgers: u32,
    ) -> Result<u64, MultisigError>;

    /// Cast an approval vote on an active proposal.
    ///
    /// If approvals reach the threshold the proposal is immediately executed.
    fn approve(
        env: Env,
        owner: Address,
        proposal_id: u64,
    ) -> Result<(), MultisigError>;

    /// Cast a rejection vote on an active proposal.
    ///
    /// If rejections reach `owners.len() - threshold + 1` the proposal is
    /// permanently rejected (it can no longer reach the approval threshold).
    fn reject(
        env: Env,
        owner: Address,
        proposal_id: u64,
    ) -> Result<(), MultisigError>;

    /// Mark an expired proposal as `Expired` to reclaim clarity in the UI.
    ///
    /// Anyone can call this after the proposal's expiry ledger has passed.
    fn expire_proposal(
        env: Env,
        caller: Address,
        proposal_id: u64,
    ) -> Result<(), MultisigError>;

    /// Return the current configuration (owners + threshold).
    fn get_config(env: Env) -> Result<MultisigConfig, MultisigError>;

    /// Return a proposal by ID.
    fn get_proposal(env: Env, proposal_id: u64) -> Result<Proposal, MultisigError>;
}
