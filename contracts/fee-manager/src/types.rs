//! Types specific to the fee-manager contract.

use soroban_sdk::{contracttype, Address};

/// The fee schedule applied platform-wide.
#[contracttype]
#[derive(Clone, Debug)]
pub struct FeeSchedule {
    /// Platform fee in basis points (applied to most operations).
    pub platform_fee_bps: i128,
    /// Fee specific to escrow releases.
    pub escrow_fee_bps: i128,
    /// Fee applied to wallet withdrawals.
    pub withdrawal_fee_bps: i128,
    /// Referral commission paid to referrers in basis points.
    pub referral_commission_bps: i128,
    /// Address that collects all fees (usually the treasury contract).
    pub fee_collector: Address,
}

/// A per-address discount rule.
///
/// The discount is expressed as a percentage off the applicable fee,
/// in basis points of the fee itself (not of the principal).
///
/// Example: a 2000 bps discount (20%) on a 100 bps fee yields an
/// effective fee of 80 bps.
#[contracttype]
#[derive(Clone, Debug)]
pub struct DiscountRule {
    /// The address that benefits from this discount.
    pub address: Address,
    /// Discount percentage in basis points of the base fee.
    /// Range: 0–10000 (0%–100%).
    pub discount_bps: i128,
    /// Ledger sequence after which this discount expires (0 = never).
    pub expires_at_ledger: u32,
}

/// The result of a fee calculation.
#[contracttype]
#[derive(Clone, Debug)]
pub struct FeeBreakdown {
    /// The principal amount the fee is calculated on.
    pub principal: i128,
    /// Base fee before any discounts.
    pub base_fee: i128,
    /// Discount amount subtracted from base fee.
    pub discount_amount: i128,
    /// Net fee after discount.
    pub net_fee: i128,
    /// Net amount received by the payer after fee deduction.
    pub net_principal: i128,
    /// Referral commission (sub-component of net_fee, paid to referrer).
    pub referral_commission: i128,
}
