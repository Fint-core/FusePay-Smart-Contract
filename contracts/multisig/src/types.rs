//! Types specific to the multisig contract.

use soroban_sdk::{contracttype, Address, Bytes, String, Vec};

pub use shared_types::ProposalState;

/// A multisig proposal awaiting owner approvals.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Proposal {
    pub proposal_id: u64,
    /// Short description of what this proposal authorises.
    pub description: String,
    /// Target contract to call on execution.
    pub target_contract: Address,
    /// ABI-encoded calldata (function name + arguments).
    pub call_data: Bytes,
    /// Number of approvals collected so far.
    pub approval_count: u32,
    /// Number of rejections collected so far.
    pub rejection_count: u32,
    pub state: ProposalState,
    pub expires_at_ledger: u32,
    pub created_at_ledger: u32,
    pub proposer: Address,
}

/// Vote record — one per (owner, proposal) pair.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Vote {
    Approve,
    Reject,
}

/// Multisig configuration stored at initialisation.
#[contracttype]
#[derive(Clone, Debug)]
pub struct MultisigConfig {
    /// List of authorised owner addresses.
    pub owners: Vec<Address>,
    /// Minimum approvals required to execute a proposal.
    pub threshold: u32,
}
