//! Input validation for the fee-manager contract.

use crate::errors::FeeManagerError;
use shared_constants::{MAX_PLATFORM_FEE_BPS, MAX_REFERRAL_COMMISSION_BPS};

/// Validate a fee rate in basis points.
pub fn validate_fee_bps(fee_bps: i128) -> Result<(), FeeManagerError> {
    if fee_bps < 0 {
        return Err(FeeManagerError::NegativeFeeRate);
    }
    if fee_bps > MAX_PLATFORM_FEE_BPS {
        return Err(FeeManagerError::FeeTooHigh);
    }
    Ok(())
}

/// Validate a referral commission rate.
pub fn validate_commission_bps(commission_bps: i128) -> Result<(), FeeManagerError> {
    if commission_bps < 0 {
        return Err(FeeManagerError::NegativeFeeRate);
    }
    if commission_bps > MAX_REFERRAL_COMMISSION_BPS {
        return Err(FeeManagerError::CommissionTooHigh);
    }
    Ok(())
}

/// Validate a discount percentage (0–10000 bps = 0%–100% of the fee).
pub fn validate_discount_bps(discount_bps: i128) -> Result<(), FeeManagerError> {
    if discount_bps < 0 || discount_bps > 10_000 {
        return Err(FeeManagerError::InvalidDiscount);
    }
    Ok(())
}

/// Validate a principal amount is strictly positive.
pub fn validate_principal(amount: i128) -> Result<(), FeeManagerError> {
    if amount <= 0 {
        return Err(FeeManagerError::InvalidAmount);
    }
    Ok(())
}
