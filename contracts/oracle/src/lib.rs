//! # fusepay-oracle
//!
//! Price feed and exchange rate oracle contract for the FusePay platform.
//!
//! ## Status: Future Module
//!
//! This contract is fully scaffolded.  Production deployment should integrate
//! with [Reflector Protocol](https://reflector.network/) or a similar
//! decentralised oracle network on Stellar.
//!
//! ## Design
//!
//! - Admin registers named feeds (e.g. "XLM/USDC")
//! - Admin authorises price providers
//! - Providers submit updated prices via `submit_price`
//! - Consumers call `get_price` — stale data is rejected automatically
//! - A staleness window is configured at init time
//!
//! ## Security notes
//!
//! - Only registered providers can submit prices
//! - Prices older than `max_staleness_ledgers` are rejected with `StalePriceFeed`
//! - Price bounds validation should be added once acceptable ranges are known

#![no_std]

mod contract;
mod errors;
mod events;
mod interface;
mod storage;
mod types;
mod utils;
mod validation;

pub use contract::OracleContract;
pub use contract::OracleContractClient;
pub use errors::OracleError;
pub use types::{OracleConfig, PriceFeed};
