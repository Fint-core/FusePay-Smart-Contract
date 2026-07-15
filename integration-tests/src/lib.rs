//! Integration test suite for FusePay contracts.
//!
//! These tests exercise cross-contract interactions and end-to-end flows
//! that cannot be covered by individual contract unit tests.

#[cfg(test)]
mod escrow_treasury_integration;

#[cfg(test)]
mod wallet_fee_integration;

#[cfg(test)]
mod full_platform_flow;
