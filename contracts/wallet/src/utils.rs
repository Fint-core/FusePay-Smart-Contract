//! Pure utility functions for the wallet contract.

use crate::errors::WalletError;
use shared_utils::{safe_add, safe_sub};

/// Add a deposit amount to the current balance.
///
/// Returns the new balance or `WalletError::ArithmeticOverflow`.
pub fn add_to_balance(balance: i128, amount: i128) -> Result<i128, WalletError> {
    safe_add(balance, amount).map_err(|_| WalletError::ArithmeticOverflow)
}

/// Subtract a withdrawal amount from the current balance.
///
/// Returns the new balance or `WalletError::ArithmeticOverflow`.
pub fn subtract_from_balance(balance: i128, amount: i128) -> Result<i128, WalletError> {
    safe_sub(balance, amount).map_err(|_| WalletError::ArithmeticOverflow)
}
