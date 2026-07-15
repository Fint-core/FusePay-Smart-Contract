//! Input validation for the wallet contract.

use crate::errors::WalletError;
use shared_constants::{MAX_TRANSACTION_AMOUNT, MIN_DEPOSIT_AMOUNT};

/// Validate that a deposit or withdrawal amount is within bounds.
pub fn validate_amount(amount: i128) -> Result<(), WalletError> {
    if amount <= 0 {
        return Err(WalletError::InvalidAmount);
    }
    if amount < MIN_DEPOSIT_AMOUNT {
        return Err(WalletError::BelowMinimumDeposit);
    }
    if amount > MAX_TRANSACTION_AMOUNT {
        return Err(WalletError::ExceedsMaximumDeposit);
    }
    Ok(())
}

/// Validate that the token address provided matches the wallet's registered token.
pub fn validate_token(
    provided: &soroban_sdk::Address,
    registered: &soroban_sdk::Address,
) -> Result<(), WalletError> {
    if provided != registered {
        return Err(WalletError::TokenMismatch);
    }
    Ok(())
}

/// Validate that the wallet is currently active (not frozen).
pub fn validate_wallet_active(is_active: bool) -> Result<(), WalletError> {
    if !is_active {
        return Err(WalletError::WalletFrozen);
    }
    Ok(())
}

/// Validate that the withdrawal amount does not exceed the current balance.
pub fn validate_sufficient_balance(balance: i128, amount: i128) -> Result<(), WalletError> {
    if amount > balance {
        return Err(WalletError::InsufficientFunds);
    }
    Ok(())
}
