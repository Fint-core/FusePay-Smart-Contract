//! Utility functions for the multisig contract.

use crate::errors::MultisigError;

/// Safely increment an approval or rejection counter.
pub fn increment_counter(count: u32) -> Result<u32, MultisigError> {
    count
        .checked_add(1)
        .ok_or(MultisigError::ArithmeticOverflow)
}
