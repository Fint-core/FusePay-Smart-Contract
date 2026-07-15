//! Public interface trait for the escrow contract.

use crate::errors::EscrowError;
use crate::types::{CreateEscrowParams, Escrow};
use soroban_sdk::{Address, Env};

pub trait EscrowInterface {
    /// Initialise the escrow contract.
    fn initialize(
        env: Env,
        admin: Address,
        treasury: Address,
        fee_bps: i128,
    ) -> Result<(), EscrowError>;

    /// Create a new escrow agreement. Does not transfer funds yet.
    ///
    /// Returns the new escrow ID.
    fn create_escrow(env: Env, params: CreateEscrowParams) -> Result<u64, EscrowError>;

    /// Deposit funds into an existing `Pending` escrow.
    ///
    /// Only the depositor can fund their own escrow.
    fn fund_escrow(env: Env, depositor: Address, escrow_id: u64) -> Result<(), EscrowError>;

    /// Release funds to the beneficiary.
    ///
    /// Can be called by the depositor (voluntary release) or the arbiter.
    /// The platform fee is deducted and sent to the treasury.
    fn release_escrow(
        env: Env,
        caller: Address,
        escrow_id: u64,
    ) -> Result<(), EscrowError>;

    /// Refund the depositor.
    ///
    /// Can be called by the beneficiary (voluntary refund) or the arbiter.
    fn refund_escrow(env: Env, caller: Address, escrow_id: u64) -> Result<(), EscrowError>;

    /// Raise a dispute on a `Funded` escrow.
    ///
    /// Either party (depositor or beneficiary) can raise a dispute.
    fn raise_dispute(env: Env, caller: Address, escrow_id: u64) -> Result<(), EscrowError>;

    /// Resolve a dispute — arbiter decides to release or refund.
    ///
    /// Only the arbiter can call this on a `Disputed` escrow.
    fn resolve_dispute(
        env: Env,
        arbiter: Address,
        escrow_id: u64,
        release: bool,
    ) -> Result<(), EscrowError>;

    /// Cancel a `Pending` escrow before it is funded.
    ///
    /// Only the depositor can cancel before funding.
    fn cancel_escrow(env: Env, depositor: Address, escrow_id: u64) -> Result<(), EscrowError>;

    /// Claim a refund on an expired `Funded` escrow.
    ///
    /// Anyone can trigger this once the expiry ledger has passed.
    fn claim_timeout_refund(
        env: Env,
        caller: Address,
        escrow_id: u64,
    ) -> Result<(), EscrowError>;

    /// Retrieve a full escrow record by ID.
    fn get_escrow(env: Env, escrow_id: u64) -> Result<Escrow, EscrowError>;

    /// Update the fee rate (admin only).
    fn update_fee(env: Env, admin: Address, new_fee_bps: i128) -> Result<(), EscrowError>;
}
