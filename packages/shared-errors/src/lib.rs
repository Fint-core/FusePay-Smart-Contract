//! # shared-errors
//!
//! Common error variants shared across FusePay smart contracts.
//!
//! Each contract also defines its own `ContractError` enum for domain-specific
//! errors. This crate holds errors that are generic enough to appear in multiple
//! contracts, preventing code duplication.
//!
//! # Error Code Ranges
//!
//! To avoid collisions when errors from different contracts are surfaced to
//! off-chain clients, each contract occupies a reserved range:
//!
//! | Contract       | Range       |
//! |----------------|-------------|
//! | shared         | 1 – 99      |
//! | wallet         | 100 – 199   |
//! | escrow         | 200 – 299   |
//! | multisig       | 300 – 399   |
//! | treasury       | 400 – 499   |
//! | access-control | 500 – 599   |
//! | fee-manager    | 600 – 699   |
//! | oracle         | 700 – 799   |

#![no_std]

use soroban_sdk::contracterror;

/// Generic errors applicable to any FusePay contract.
///
/// These occupy the range 1–99 and should not be re-defined in individual
/// contracts. Individual contracts embed their own error enums for
/// domain-specific failures.
#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SharedError {
    /// The caller is not authorised to perform this operation.
    Unauthorized = 1,

    /// A required parameter was missing or zero.
    InvalidInput = 2,

    /// An arithmetic operation would overflow or underflow.
    ArithmeticOverflow = 3,

    /// The requested resource was not found in storage.
    NotFound = 4,

    /// The operation is not allowed in the current state.
    InvalidState = 5,

    /// The contract has already been initialised and cannot be re-initialised.
    AlreadyInitialized = 6,

    /// The contract has not yet been initialised.
    NotInitialized = 7,

    /// A zero amount was provided where a positive amount is required.
    ZeroAmount = 8,

    /// The provided address is the zero address or otherwise invalid.
    InvalidAddress = 9,

    /// The operation deadline / expiry has passed.
    Expired = 10,

    /// The operation has already been completed and cannot be repeated.
    AlreadyCompleted = 11,

    /// The caller's account does not have sufficient funds.
    InsufficientFunds = 12,

    /// A value exceeded the configured maximum limit.
    ExceedsMaximum = 13,

    /// A value was below the configured minimum limit.
    BelowMinimum = 14,

    /// The provided token address is not supported by this contract.
    UnsupportedToken = 15,
}
