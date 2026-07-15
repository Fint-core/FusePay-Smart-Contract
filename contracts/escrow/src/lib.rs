//! # fusepay-escrow
//!
//! Trustless escrow contract for the FusePay platform.
//!
//! ## Lifecycle
//!
//! ```text
//!  create_escrow()
//!       │
//!       ▼
//!   [Pending] ──── cancel_escrow() ──────► [Cancelled]
//!       │
//!  fund_escrow()
//!       │
//!       ▼
//!   [Funded] ──── release_escrow() ──────► [Released]
//!       │      ├─ refund_escrow()  ──────► [Refunded]
//!       │      ├─ claim_timeout_refund() ► [Expired]   (after expiry ledger)
//!       │
//!   raise_dispute()
//!       │
//!       ▼
//!   [Disputed] ─── resolve_dispute(release=true) ── ► [Released]
//!              └── resolve_dispute(release=false) ─── ► [Refunded]
//! ```
//!
//! ## Security Notes
//!
//! - State is updated before any token transfers (Checks-Effects-Interactions)
//! - `require_auth` is called on every caller before state reads
//! - Expired escrows can only be refunded — never released after expiry

#![no_std]

mod contract;
mod errors;
mod events;
mod interface;
mod storage;
mod types;
mod utils;
mod validation;

pub use contract::EscrowContract;
pub use contract::EscrowContractClient;
pub use errors::EscrowError;
pub use types::{CreateEscrowParams, Escrow, EscrowConfig, EscrowState};
