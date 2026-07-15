//! Access-control contract error codes.
//!
//! Error codes occupy the range 500–599 per the FusePay error-range convention.

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum AccessControlError {
    /// The contract has already been initialised.
    AlreadyInitialized = 500,

    /// The contract has not been initialised yet.
    NotInitialized = 501,

    /// The caller does not have the required role.
    Unauthorized = 502,

    /// The provided address is the zero / invalid address.
    InvalidAddress = 503,

    /// Attempted to grant a role that the address already holds.
    RoleAlreadyGranted = 504,

    /// Attempted to revoke a role the address does not hold.
    RoleNotGranted = 505,

    /// The Owner role cannot be revoked directly; use `transfer_ownership`.
    CannotRevokeOwner = 506,

    /// Ownership cannot be transferred to the zero / current owner address.
    InvalidOwnerAddress = 507,
}
