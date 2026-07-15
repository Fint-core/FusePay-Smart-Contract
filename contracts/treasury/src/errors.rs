//! Treasury contract error codes (range 400–499).

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TreasuryError {
    AlreadyInitialized    = 400,
    NotInitialized        = 401,
    Unauthorized          = 402,
    InvalidAmount         = 403,
    InsufficientBalance   = 404,
    /// Withdrawal amount exceeds the single-approver threshold and
    /// must go through multisig.
    RequiresMultisig      = 405,
    ArithmeticOverflow    = 406,
    /// The deposit token is not registered in the treasury.
    UnsupportedToken      = 407,
    /// The recipient address is invalid.
    InvalidRecipient      = 408,
    /// Zero-amount operation attempted.
    ZeroAmount            = 409,
    /// Distribution amounts do not sum correctly.
    InvalidDistribution   = 410,
}
