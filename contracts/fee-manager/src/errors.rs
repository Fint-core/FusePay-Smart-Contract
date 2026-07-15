//! Fee-manager contract error codes (range 600–699).

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum FeeManagerError {
    AlreadyInitialized       = 600,
    NotInitialized           = 601,
    Unauthorized             = 602,
    InvalidAmount            = 603,
    /// A fee rate exceeds the platform maximum.
    FeeTooHigh               = 604,
    /// A fee rate is negative.
    NegativeFeeRate          = 605,
    /// A discount percentage exceeds 100%.
    InvalidDiscount          = 606,
    /// Referral commission rate exceeds the maximum.
    CommissionTooHigh        = 607,
    /// The referrer address is the same as the fee payer.
    SelfReferral             = 608,
    ArithmeticOverflow       = 609,
    /// A discount rule for this address already exists.
    DiscountAlreadyExists    = 610,
    /// No discount rule found for this address.
    DiscountNotFound         = 611,
}
