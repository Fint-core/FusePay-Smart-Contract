//! Escrow contract error codes (range 200–299).

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum EscrowError {
    /// Contract has already been initialised.
    AlreadyInitialized = 200,

    /// Contract has not been initialised.
    NotInitialized = 201,

    /// The caller is not authorised for this operation.
    Unauthorized = 202,

    /// The escrow ID does not exist.
    EscrowNotFound = 203,

    /// The escrow is not in the expected state for this operation.
    InvalidState = 204,

    /// The provided amount is zero or negative.
    InvalidAmount = 205,

    /// The amount is below the minimum escrow amount.
    BelowMinimumAmount = 206,

    /// The timeout exceeds the maximum allowed escrow duration.
    TimeoutTooLong = 207,

    /// The timeout has already passed at creation time.
    InvalidTimeout = 208,

    /// The escrow has expired and can only be refunded.
    EscrowExpired = 209,

    /// The escrow has not yet expired — timeout refund not available yet.
    EscrowNotExpired = 210,

    /// Arithmetic operation overflowed.
    ArithmeticOverflow = 211,

    /// The depositor and beneficiary cannot be the same address.
    SameParty = 212,

    /// The arbiter address is invalid.
    InvalidArbiter = 213,

    /// Fee calculation resulted in a fee larger than the principal.
    FeeTooLarge = 214,
}
