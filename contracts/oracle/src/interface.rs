//! Public interface for the oracle contract.

use crate::errors::OracleError;
use crate::types::PriceFeed;
use soroban_sdk::{Address, Env, String};

pub trait OracleInterface {
    /// Initialise the oracle with an admin and staleness window.
    fn initialize(
        env: Env,
        admin: Address,
        max_staleness_ledgers: u32,
    ) -> Result<(), OracleError>;

    /// Register a new price feed (admin only).
    fn register_feed(
        env: Env,
        admin: Address,
        feed_id: String,
        base_asset: String,
        quote_asset: String,
        decimals: u32,
    ) -> Result<(), OracleError>;

    /// Submit an updated price for a feed (authorised provider only).
    fn submit_price(
        env: Env,
        provider: Address,
        feed_id: String,
        price: i128,
    ) -> Result<(), OracleError>;

    /// Get the latest price for a feed.
    ///
    /// Returns `StalePriceFeed` if the price has not been updated within
    /// the configured staleness window.
    fn get_price(env: Env, feed_id: String) -> Result<PriceFeed, OracleError>;

    /// Add an authorised price provider (admin only).
    fn add_provider(env: Env, admin: Address, provider: Address) -> Result<(), OracleError>;

    /// Remove a price provider (admin only).
    fn remove_provider(env: Env, admin: Address, provider: Address) -> Result<(), OracleError>;
}
