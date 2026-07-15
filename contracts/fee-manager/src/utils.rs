//! Fee calculation utilities for the fee-manager contract.

use crate::errors::FeeManagerError;
use crate::types::{DiscountRule, FeeBreakdown, FeeSchedule};
use shared_constants::FEE_DENOMINATOR;
use shared_utils::calculate_fee;
use soroban_sdk::Env;

/// Calculate the full fee breakdown for a principal amount.
///
/// Applies any active discount rule for `payer_address` and computes
/// the referral commission as a sub-component of the net fee.
pub fn calculate_fee_breakdown(
    env: &Env,
    principal: i128,
    fee_bps: i128,
    schedule: &FeeSchedule,
    discount: Option<&DiscountRule>,
    has_referrer: bool,
) -> Result<FeeBreakdown, FeeManagerError> {
    // 1. Base fee
    let base_fee =
        calculate_fee(principal, fee_bps).map_err(|_| FeeManagerError::ArithmeticOverflow)?;

    // 2. Discount
    let discount_amount = if let Some(rule) = discount {
        // Check expiry — expired discounts are treated as absent.
        let is_valid = rule.expires_at_ledger == 0
            || env.ledger().sequence() < rule.expires_at_ledger;

        if is_valid && rule.discount_bps > 0 {
            base_fee
                .checked_mul(rule.discount_bps)
                .and_then(|p| p.checked_div(FEE_DENOMINATOR))
                .ok_or(FeeManagerError::ArithmeticOverflow)?
        } else {
            0
        }
    } else {
        0
    };

    let net_fee = base_fee
        .checked_sub(discount_amount)
        .ok_or(FeeManagerError::ArithmeticOverflow)?;

    // 3. Referral commission (sub-component of net fee, not additional charge)
    let referral_commission = if has_referrer && schedule.referral_commission_bps > 0 {
        net_fee
            .checked_mul(schedule.referral_commission_bps)
            .and_then(|p| p.checked_div(FEE_DENOMINATOR))
            .ok_or(FeeManagerError::ArithmeticOverflow)?
    } else {
        0
    };

    let net_principal = principal
        .checked_sub(net_fee)
        .ok_or(FeeManagerError::ArithmeticOverflow)?;

    Ok(FeeBreakdown {
        principal,
        base_fee,
        discount_amount,
        net_fee,
        net_principal,
        referral_commission,
    })
}
