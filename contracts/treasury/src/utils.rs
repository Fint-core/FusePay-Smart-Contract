//! Utility functions for the treasury contract.

use crate::errors::TreasuryError;
use crate::types::DistributionEntry;
use shared_utils::{safe_add, safe_sub};
use soroban_sdk::Vec;

/// Add to a balance, returning `TreasuryError::ArithmeticOverflow` on overflow.
pub fn add_balance(balance: i128, amount: i128) -> Result<i128, TreasuryError> {
    safe_add(balance, amount).map_err(|_| TreasuryError::ArithmeticOverflow)
}

/// Subtract from a balance, returning `TreasuryError::ArithmeticOverflow` on underflow.
pub fn sub_balance(balance: i128, amount: i128) -> Result<i128, TreasuryError> {
    safe_sub(balance, amount).map_err(|_| TreasuryError::ArithmeticOverflow)
}

/// Sum all amounts in a distribution entry list.
///
/// Returns the total or `TreasuryError::ArithmeticOverflow`.
pub fn sum_distribution(entries: &Vec<DistributionEntry>) -> Result<i128, TreasuryError> {
    let mut total: i128 = 0;
    for entry in entries.iter() {
        total = add_balance(total, entry.amount)?;
    }
    Ok(total)
}
