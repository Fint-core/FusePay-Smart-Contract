//! # fusepay-wallet
//!
//! Wallet contract for the FusePay platform.
//!
//! ## Responsibilities
//!
//! - Create one wallet per (owner, token) pair
//! - Accept token deposits from the owner
//! - Process token withdrawals to any recipient (owner-authorised)
//! - Track running totals of deposits and withdrawals
//! - Allow the platform admin to freeze/unfreeze wallets
//!
//! ## Security Notes
//!
//! - All state is updated before external token transfers (CEI pattern)
//! - All amounts are validated before modifying state
//! - `require_auth` is called on the owner for every mutating operation
//! - The admin can freeze wallets but cannot withdraw funds

#![no_std]

mod contract;
mod errors;
mod events;
mod interface;
mod storage;
mod types;
mod utils;
mod validation;

pub use contract::WalletContract;
pub use contract::WalletContractClient;
pub use errors::WalletError;
pub use types::Wallet;
