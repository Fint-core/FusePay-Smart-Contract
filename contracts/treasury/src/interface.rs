//! Public interface trait for the treasury contract.

use crate::errors::TreasuryError;
use crate::types::{DistributionEntry, TokenLedger, TreasuryConfig};
use soroban_sdk::{Address, Env, Vec};

pub trait TreasuryInterface {
    /// Initialise the treasury.
    fn initialize(
        env: Env,
        admin: Address,
        multisig_contract: Address,
        multisig_threshold: i128,
    ) -> Result<(), TreasuryError>;

    /// Deposit tokens into the treasury.
    ///
    /// Called by fee-manager, escrow, or any authorised contract.
    /// Caller must have approved a transfer from `sender` first.
    fn deposit(
        env: Env,
        sender: Address,
        token: Address,
        amount: i128,
    ) -> Result<(), TreasuryError>;

    /// Withdraw tokens from the treasury to a recipient.
    ///
    /// Withdrawals below the multisig threshold can be authorised by the
    /// admin alone.  At or above the threshold, the admin must have
    /// first obtained approval from the multisig contract (the call
    /// will revert with `RequiresMultisig` if not).
    fn withdraw(
        env: Env,
        admin: Address,
        token: Address,
        amount: i128,
        recipient: Address,
    ) -> Result<(), TreasuryError>;

    /// Batch distribute tokens to multiple recipients in a single transaction.
    ///
    /// The sum of all amounts must not exceed the available balance.
    /// Subject to the same multisig threshold as `withdraw`.
    fn distribute(
        env: Env,
        admin: Address,
        token: Address,
        entries: Vec<DistributionEntry>,
    ) -> Result<(), TreasuryError>;

    /// Returns the token ledger (balance + accounting totals) for a token.
    fn get_token_ledger(env: Env, token: Address) -> Result<TokenLedger, TreasuryError>;

    /// Returns the current treasury configuration.
    fn get_config(env: Env) -> Result<TreasuryConfig, TreasuryError>;

    /// Update the multisig threshold (admin only).
    fn update_multisig_threshold(
        env: Env,
        admin: Address,
        new_threshold: i128,
    ) -> Result<(), TreasuryError>;
}
