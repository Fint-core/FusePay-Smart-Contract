//! # fusepay-multisig
//!
//! Multi-signature approval contract for the FusePay platform.
//!
//! ## Overview
//!
//! Any privileged operation that exceeds a configured risk threshold (e.g.
//! large treasury withdrawals, contract upgrades, owner changes) must first
//! be approved by a quorum of configured owners through this contract.
//!
//! ## Proposal lifecycle
//!
//! ```text
//!  submit_proposal()
//!        │
//!        ▼
//!    [Active] ──── approve() × threshold ──────► [Executed]
//!        │      ├─ reject()  × blocking  ──────► [Rejected]
//!        └────── expire_proposal() (after expiry) ► [Expired]
//! ```
//!
//! ## Integration pattern
//!
//! Other contracts check `MultisigContract::is_approved(proposal_id)` before
//! executing a privileged action.  The proposal's `call_data` field carries
//! the parameters of the intended operation as an opaque byte string that the
//! caller is responsible for encoding and validating.
//!
//! ## Security notes
//!
//! - Each owner can vote exactly once per proposal.
//! - A proposal cannot be executed after its expiry ledger, even if it
//!   previously reached the threshold.
//! - The owner list and threshold can only be changed by re-initialising a
//!   new multisig instance (governance upgrade pattern).

#![no_std]

mod contract;
mod errors;
mod events;
mod interface;
mod storage;
mod types;
mod utils;
mod validation;

pub use contract::MultisigContract;
pub use contract::MultisigContractClient;
pub use errors::MultisigError;
pub use types::{MultisigConfig, Proposal, ProposalState, Vote};
