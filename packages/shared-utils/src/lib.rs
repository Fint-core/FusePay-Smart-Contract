//! # shared-utils
//!
//! Pure utility functions shared across all FusePay contracts.
//!
//! Functions in this module are side-effect-free (no ledger reads or writes)
//! and can be unit-tested without a Soroban environment.

#![no_std]

use shared_constants::FEE_DENOMINATOR;
use shared_errors::SharedError;
use soroban_sdk::Env;

// ── Fee calculation ────────────────────────────────────────────────────────

/// Calculate a fee amount from a principal and a basis-point rate.
///
/// # Arguments
/// * `amount`  – The principal amount (in stroops or smallest token unit).
/// * `fee_bps` – The fee rate in basis points (1 bp = 0.01%).
///
/// # Returns
/// The fee amount, or `SharedError::ArithmeticOverflow` if the multiplication
/// would overflow an i128.
///
/// # Example
/// ```
/// // 1,000,000 stroops at 50 bps = 5,000 stroops fee
/// let fee = calculate_fee(1_000_000, 50).unwrap();
/// assert_eq!(fee, 5_000);
/// ```
pub fn calculate_fee(amount: i128, fee_bps: i128) -> Result<i128, SharedError> {
    amount
        .checked_mul(fee_bps)
        .and_then(|product| product.checked_div(FEE_DENOMINATOR))
        .ok_or(SharedError::ArithmeticOverflow)
}

/// Subtract a fee from an amount, returning `(net_amount, fee_amount)`.
///
/// # Errors
/// Returns `SharedError::ArithmeticOverflow` if multiplication overflows.
/// Returns `SharedError::InsufficientFunds` if the fee exceeds the amount.
pub fn deduct_fee(amount: i128, fee_bps: i128) -> Result<(i128, i128), SharedError> {
    let fee = calculate_fee(amount, fee_bps)?;
    let net = amount
        .checked_sub(fee)
        .ok_or(SharedError::InsufficientFunds)?;
    Ok((net, fee))
}

// ── Safe arithmetic helpers ────────────────────────────────────────────────

/// Safe addition — returns `SharedError::ArithmeticOverflow` on overflow.
#[inline]
pub fn safe_add(a: i128, b: i128) -> Result<i128, SharedError> {
    a.checked_add(b).ok_or(SharedError::ArithmeticOverflow)
}

/// Safe subtraction — returns `SharedError::ArithmeticOverflow` on underflow.
#[inline]
pub fn safe_sub(a: i128, b: i128) -> Result<i128, SharedError> {
    a.checked_sub(b).ok_or(SharedError::ArithmeticOverflow)
}

/// Safe multiplication — returns `SharedError::ArithmeticOverflow` on overflow.
#[inline]
pub fn safe_mul(a: i128, b: i128) -> Result<i128, SharedError> {
    a.checked_mul(b).ok_or(SharedError::ArithmeticOverflow)
}

/// Safe division — returns `SharedError::ArithmeticOverflow` on divide-by-zero.
#[inline]
pub fn safe_div(a: i128, b: i128) -> Result<i128, SharedError> {
    a.checked_div(b).ok_or(SharedError::ArithmeticOverflow)
}

// ── Amount validation ──────────────────────────────────────────────────────

/// Assert that an amount is strictly positive.
///
/// Returns `SharedError::ZeroAmount` if `amount <= 0`.
/// Returns `SharedError::BelowMinimum` if `amount < min`.
/// Returns `SharedError::ExceedsMaximum` if `amount > max`.
pub fn validate_amount(amount: i128, min: i128, max: i128) -> Result<(), SharedError> {
    if amount <= 0 {
        return Err(SharedError::ZeroAmount);
    }
    if amount < min {
        return Err(SharedError::BelowMinimum);
    }
    if amount > max {
        return Err(SharedError::ExceedsMaximum);
    }
    Ok(())
}

// ── Ledger helpers ─────────────────────────────────────────────────────────

/// Returns the current ledger sequence number.
#[inline]
pub fn current_ledger(env: &Env) -> u32 {
    env.ledger().sequence()
}

/// Returns `true` if the given deadline ledger has already passed.
#[inline]
pub fn is_expired(env: &Env, expires_at_ledger: u32) -> bool {
    env.ledger().sequence() >= expires_at_ledger
}

/// Compute an absolute expiry ledger from the current ledger plus a delta.
///
/// Returns `SharedError::ArithmeticOverflow` if the addition overflows u32.
pub fn expiry_ledger(env: &Env, timeout_ledgers: u32) -> Result<u32, SharedError> {
    current_ledger(env)
        .checked_add(timeout_ledgers)
        .ok_or(SharedError::ArithmeticOverflow)
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fee_standard() {
        // 1,000,000 stroops × 50 bps / 10,000 = 5,000 stroops
        assert_eq!(calculate_fee(1_000_000, 50).unwrap(), 5_000);
    }

    #[test]
    fn test_calculate_fee_zero_rate() {
        assert_eq!(calculate_fee(1_000_000, 0).unwrap(), 0);
    }

    #[test]
    fn test_calculate_fee_zero_amount() {
        assert_eq!(calculate_fee(0, 50).unwrap(), 0);
    }

    #[test]
    fn test_deduct_fee_returns_net_and_fee() {
        let (net, fee) = deduct_fee(1_000_000, 50).unwrap();
        assert_eq!(fee, 5_000);
        assert_eq!(net, 995_000);
    }

    #[test]
    fn test_safe_add_overflow() {
        let result = safe_add(i128::MAX, 1);
        assert_eq!(result, Err(SharedError::ArithmeticOverflow));
    }

    #[test]
    fn test_safe_sub_underflow() {
        let result = safe_sub(0, 1);
        assert_eq!(result, Err(SharedError::ArithmeticOverflow));
    }

    #[test]
    fn test_validate_amount_zero() {
        assert_eq!(validate_amount(0, 1, 1_000), Err(SharedError::ZeroAmount));
    }

    #[test]
    fn test_validate_amount_below_min() {
        assert_eq!(
            validate_amount(5, 10, 1_000),
            Err(SharedError::BelowMinimum)
        );
    }

    #[test]
    fn test_validate_amount_exceeds_max() {
        assert_eq!(
            validate_amount(2_000, 10, 1_000),
            Err(SharedError::ExceedsMaximum)
        );
    }

    #[test]
    fn test_validate_amount_valid() {
        assert_eq!(validate_amount(500, 10, 1_000), Ok(()));
    }
}
