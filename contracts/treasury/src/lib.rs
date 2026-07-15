//! # fusepay-treasury
//!
//! Platform treasury contract for the FusePay platform.
//!
//! ## Responsibilities
//!
//! - Receive fee deposits from the escrow and fee-manager contracts
//! - Track per-token accounting (total deposited, total distributed, available)
//! - Allow the admin to withdraw or distribute funds
//! - Enforce a configurable multisig threshold for large withdrawals
//!
//! ## Security notes
//!
//! - All state is updated before external token transfers (CEI pattern)
//! - Withdrawals at or above `multisig_threshold` return `RequiresMultisig`;
//!   the caller must obtain multisig approval and embed it in the call flow
//! - The admin cannot change the multisig contract address without redeploying

#![no_std]

mod contract;
mod errors;
mod events;
mod interface;
mod storage;
mod types;
mod utils;
mod validation;

pub use contract::TreasuryContract;
pub use contract::TreasuryContractClient;
pub use errors::TreasuryError;
pub use types::{DistributionEntry, TokenLedger, TreasuryConfig};
