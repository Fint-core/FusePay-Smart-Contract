//! Types specific to the treasury contract.

use soroban_sdk::{contracttype, Address};

/// Running accounting totals for a single token.
#[contracttype]
#[derive(Clone, Debug)]
pub struct TokenLedger {
    /// Token contract address.
    pub token: Address,
    /// Cumulative amount ever deposited (monotonically increasing).
    pub total_deposited: i128,
    /// Cumulative amount ever withdrawn / distributed (monotonically increasing).
    pub total_distributed: i128,
    /// Current available balance (deposited - distributed).
    pub available: i128,
}

/// Configuration stored at initialisation.
#[contracttype]
#[derive(Clone, Debug)]
pub struct TreasuryConfig {
    /// Address authorised to perform single-signer withdrawals below the threshold.
    pub admin: Address,
    /// Address of the multisig contract used for large withdrawals.
    pub multisig_contract: Address,
    /// Withdrawals at or above this amount (in stroops) require multisig approval.
    pub multisig_threshold: i128,
}

/// A single line item in a distribution (recipient → amount).
#[contracttype]
#[derive(Clone, Debug)]
pub struct DistributionEntry {
    pub recipient: Address,
    pub amount: i128,
}
