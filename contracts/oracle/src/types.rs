//! Types for the oracle contract.

use soroban_sdk::{contracttype, Address, String};

/// A price feed record for an asset pair.
#[contracttype]
#[derive(Clone, Debug)]
pub struct PriceFeed {
    /// Feed identifier, e.g. "XLM/USDC".
    pub feed_id: String,
    /// Base asset symbol.
    pub base_asset: String,
    /// Quote asset symbol.
    pub quote_asset: String,
    /// Latest price: quote units per 1 base unit, scaled by `decimals`.
    /// e.g. price = 1_250_000 with decimals = 6 means 1.25 USDC per XLM.
    pub price: i128,
    /// Number of decimal places in the price.
    pub decimals: u32,
    /// Ledger sequence when this price was last updated.
    pub updated_at_ledger: u32,
    /// Address of the provider that submitted this price.
    pub submitted_by: Address,
}

/// Oracle configuration.
#[contracttype]
#[derive(Clone, Debug)]
pub struct OracleConfig {
    /// Contract admin — can add/remove providers and feeds.
    pub admin: Address,
    /// Maximum ledgers a price can be old before it is considered stale.
    pub max_staleness_ledgers: u32,
}
