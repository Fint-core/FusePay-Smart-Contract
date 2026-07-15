//! # fusepay-fee-manager
//!
//! Fee calculation, discount rules, and referral commission management
//! for the FusePay platform.
//!
//! ## Fee types
//!
//! | Fee | Applies to |
//! |-----|------------|
//! | Platform fee | General platform operations |
//! | Escrow fee | Escrow releases |
//! | Withdrawal fee | Wallet withdrawals |
//!
//! All rates are stored in basis points (1 bp = 0.01%).
//!
//! ## Discount rules
//!
//! Admins can assign per-address discount rules.  A discount is expressed
//! as a percentage reduction of the applicable fee (in bps of the fee, not
//! the principal).  Discounts can be given an expiry ledger after which they
//! are silently ignored.
//!
//! ## Referral commissions
//!
//! When a referrer address is provided, a commission (in bps of the net fee)
//! is computed and returned in the `FeeBreakdown`.  The calling contract is
//! responsible for routing the commission to the referrer — this contract
//! only calculates it.

#![no_std]

mod contract;
mod errors;
mod events;
mod interface;
mod storage;
mod types;
mod utils;
mod validation;

pub use contract::FeeManagerContract;
pub use contract::FeeManagerContractClient;
pub use errors::FeeManagerError;
pub use types::{DiscountRule, FeeBreakdown, FeeSchedule};
