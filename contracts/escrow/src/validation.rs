//! Input validation for the escrow contract.

use crate::errors::EscrowError;
use crate::types::EscrowState;
use shared_constants::{
    DEFAULT_ESCROW_TIMEOUT_LEDGERS, MAX_ESCROW_TIMEOUT_LEDGERS, MIN_ESCROW_AMOUNT,
};
use soroban_sdk::{Address, Env};

/// Validate the escrow amount meets the minimum.
pub fn validate_amount(amount: i128) -> Result<(), EscrowError> {
    if amount <= 0 {
        return Err(EscrowError::InvalidAmount);
    }
    if amount < MIN_ESCROW_AMOUNT {
        return Err(EscrowError::BelowMinimumAmount);
    }
    Ok(())
}

/// Validate that depositor and beneficiary are different parties.
pub fn validate_parties(depositor: &Address, beneficiary: &Address) -> Result<(), EscrowError> {
    if depositor == beneficiary {
        return Err(EscrowError::SameParty);
    }
    Ok(())
}

/// Validate and normalise the timeout.
///
/// Returns the final timeout (using default if 0 was provided) or an error
/// if the requested timeout exceeds the maximum.
pub fn validate_timeout(timeout_ledgers: u32) -> Result<u32, EscrowError> {
    if timeout_ledgers == 0 {
        return Ok(DEFAULT_ESCROW_TIMEOUT_LEDGERS);
    }
    if timeout_ledgers > MAX_ESCROW_TIMEOUT_LEDGERS {
        return Err(EscrowError::TimeoutTooLong);
    }
    Ok(timeout_ledgers)
}

/// Assert the escrow is in the expected state.
pub fn require_state(actual: &EscrowState, expected: &EscrowState) -> Result<(), EscrowError> {
    if actual != expected {
        return Err(EscrowError::InvalidState);
    }
    Ok(())
}

/// Assert the escrow has NOT expired yet.
pub fn require_not_expired(env: &Env, expires_at_ledger: u32) -> Result<(), EscrowError> {
    if env.ledger().sequence() >= expires_at_ledger {
        return Err(EscrowError::EscrowExpired);
    }
    Ok(())
}

/// Assert the escrow HAS expired (for timeout-based refunds).
pub fn require_expired(env: &Env, expires_at_ledger: u32) -> Result<(), EscrowError> {
    if env.ledger().sequence() < expires_at_ledger {
        return Err(EscrowError::EscrowNotExpired);
    }
    Ok(())
}
