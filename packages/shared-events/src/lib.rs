//! # shared-events
//!
//! Common event emission helpers shared across FusePay contracts.
//!
//! Each contract has its own `events.rs` for domain events. This crate provides
//! helpers for cross-cutting events such as contract initialisation and
//! ownership transfers, as well as the standard topic-naming convention used
//! throughout the platform.
//!
//! ## Event Format
//!
//! Every FusePay event follows this Soroban event structure:
//! - **topics[0]**: contract domain prefix symbol, e.g. `Symbol::new("fusepay")`
//! - **topics[1]**: event name symbol, e.g. `Symbol::new("wallet_created")`
//! - **topics[2..n]**: optional indexed fields (address, id, etc.)
//! - **data**: tuple or struct with full event payload
//!
//! This structure makes events efficiently filterable by both contract domain
//! and event type in off-chain indexers.

#![no_std]

use soroban_sdk::{symbol_short, Address, Env, Symbol};

// ── Topic helpers ──────────────────────────────────────────────────────────

/// The top-level domain symbol used as `topics[0]` in all FusePay events.
/// Off-chain indexers can filter on this symbol to collect all FusePay events.
pub fn fusepay_topic() -> Symbol {
    symbol_short!("fusepay")
}

// ── Initialisation events ──────────────────────────────────────────────────

/// Emitted when any FusePay contract is successfully initialised.
///
/// # Topics
/// - `"fusepay"`
/// - `"initialized"`
/// - contract address (admin)
///
/// # Data
/// - `(contract_name: Symbol, version: u32)`
pub fn emit_initialized(env: &Env, admin: &Address, contract_name: Symbol, version: u32) {
    env.events().publish(
        (fusepay_topic(), symbol_short!("init"), admin.clone()),
        (contract_name, version),
    );
}

// ── Ownership transfer events ──────────────────────────────────────────────

/// Emitted when contract ownership is transferred.
///
/// # Topics
/// - `"fusepay"`
/// - `"owner_xfer"`
/// - previous owner address
///
/// # Data
/// - `(new_owner: Address)`
pub fn emit_ownership_transferred(env: &Env, previous_owner: &Address, new_owner: &Address) {
    env.events().publish(
        (
            fusepay_topic(),
            symbol_short!("ownr_xfer"),
            previous_owner.clone(),
        ),
        (new_owner.clone(),),
    );
}

// ── Contract upgrade events ────────────────────────────────────────────────

/// Emitted when a contract's wasm hash is upgraded.
///
/// # Topics
/// - `"fusepay"`
/// - `"upgraded"`
/// - upgrader address
///
/// # Data
/// - `(new_wasm_hash: soroban_sdk::BytesN<32>)`
pub fn emit_contract_upgraded(
    env: &Env,
    upgrader: &Address,
    new_wasm_hash: soroban_sdk::BytesN<32>,
) {
    env.events().publish(
        (
            fusepay_topic(),
            symbol_short!("upgraded"),
            upgrader.clone(),
        ),
        (new_wasm_hash,),
    );
}
