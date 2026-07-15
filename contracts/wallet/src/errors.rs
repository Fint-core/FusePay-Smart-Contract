//! Wallet contract error codes (range 100–199).

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WalletError {
    /// Contract has already been initialised.
    AlreadyInitialized = 100,

    /// Contract has not been initialised.
    NotInitialized = 101,

    /// Caller is not the wallet owner.
    Unauthorized = 102,

    /// Deposit or withdrawal amount is zero or negative.
    InvalidAmount = 103,

    /// Withdrawal amount exceeds current balance.
    InsufficientFunds = 104,

    /// The wallet is frozen and cannot process transactions.
    WalletFrozen = 105,

    /// Provided token address is not the wallet's registered token.
    TokenMismatch = 106,

    /// An arithmetic operation overflowed.
    ArithmeticOverflow = 107,

    /// Attempted to create a second wallet for the same owner+token pair.
    WalletAlreadyExists = 108,

    /// The provided recipient address is invalid.
    InvalidRecipient = 109,

    /// Deposit amount is below the minimum allowed.
    BelowMinimumDeposit = 110,

    /// Deposit amount exceeds the single-transaction maximum.
    ExceedsMaximumDeposit = 111,
}
