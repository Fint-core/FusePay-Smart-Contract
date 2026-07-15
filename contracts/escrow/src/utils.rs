//! Utility functions for the escrow contract.

use crate::errors::EscrowError;
use shared_constants::MAX_PLATFORM_FEE_BPS;
use shared_utils::calculate_fee;

/// Compute the fee and net amounts for a given principal and fee rate.
///
/// Returns `(net_amount, fee_amount)`.
///
/// Validates that the fee rate does not exceed the platform maximum and
/// that the resulting fee does not exceed the principal.
pub fn compute_escrow_amounts(
    amount: i128,
    fee_bps: i128,
) -> Result<(i128, i128), EscrowError> {
    if fee_bps > MAX_PLATFORM_FEE_BPS {
        return Err(EscrowError::FeeTooLarge);
    }

    let fee = calculate_fee(amount, fee_bps).map_err(|_| EscrowError::ArithmeticOverflow)?;

    if fee >= amount {
        return Err(EscrowError::FeeTooLarge);
    }

    let net = amount
        .checked_sub(fee)
        .ok_or(EscrowError::ArithmeticOverflow)?;

    Ok((net, fee))
}
