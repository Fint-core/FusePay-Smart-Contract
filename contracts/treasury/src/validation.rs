//! Input validation for the treasury contract.

use crate::errors::TreasuryError;

pub fn validate_amount(amount: i128) -> Result<(), TreasuryError> {
    if amount <= 0 {
        return Err(TreasuryError::ZeroAmount);
    }
    Ok(())
}

pub fn validate_sufficient_balance(available: i128, amount: i128) -> Result<(), TreasuryError> {
    if amount > available {
        return Err(TreasuryError::InsufficientBalance);
    }
    Ok(())
}

/// Validate that a withdrawal does NOT require multisig (i.e. is below threshold).
/// Returns `TreasuryError::RequiresMultisig` if the amount is at or above threshold.
pub fn validate_single_signer_limit(
    amount: i128,
    threshold: i128,
) -> Result<(), TreasuryError> {
    if amount >= threshold {
        return Err(TreasuryError::RequiresMultisig);
    }
    Ok(())
}
