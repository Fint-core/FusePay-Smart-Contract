//! Oracle contract implementation.
//!
//! # Status: Future module
//!
//! This contract is fully scaffolded and functional. Integration with
//! external price feed providers (e.g. Reflector Protocol on Stellar) and
//! multi-provider aggregation logic should be added in a future release.

use crate::errors::OracleError;
use crate::events;
use crate::storage;
use crate::types::{OracleConfig, PriceFeed};
use crate::validation;
use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct OracleContract;

#[contractimpl]
impl OracleContract {
    // ── Initialisation ─────────────────────────────────────────────────

    pub fn initialize(
        env: Env,
        admin: Address,
        max_staleness_ledgers: u32,
    ) -> Result<(), OracleError> {
        if storage::is_initialized(&env) {
            return Err(OracleError::AlreadyInitialized);
        }

        admin.require_auth();
        validation::validate_staleness(max_staleness_ledgers)?;

        let config = OracleConfig {
            admin,
            max_staleness_ledgers,
        };
        storage::set_config(&env, &config);
        storage::set_initialized(&env);
        Ok(())
    }

    // ── Feed registration ──────────────────────────────────────────────

    pub fn register_feed(
        env: Env,
        admin: Address,
        feed_id: String,
        base_asset: String,
        quote_asset: String,
        decimals: u32,
    ) -> Result<(), OracleError> {
        if !storage::is_initialized(&env) {
            return Err(OracleError::NotInitialized);
        }

        admin.require_auth();

        let config = storage::get_config(&env).ok_or(OracleError::NotInitialized)?;
        if admin != config.admin {
            return Err(OracleError::Unauthorized);
        }

        if storage::feed_exists(&env, &feed_id) {
            return Err(OracleError::FeedAlreadyExists);
        }

        let feed = PriceFeed {
            feed_id: feed_id.clone(),
            base_asset,
            quote_asset,
            price: 0, // not yet set
            decimals,
            updated_at_ledger: 0,
            submitted_by: admin.clone(),
        };

        storage::save_feed(&env, &feed);
        events::emit_feed_registered(&env, &feed_id, &admin);
        Ok(())
    }

    // ── Price submission ───────────────────────────────────────────────

    pub fn submit_price(
        env: Env,
        provider: Address,
        feed_id: String,
        price: i128,
    ) -> Result<(), OracleError> {
        if !storage::is_initialized(&env) {
            return Err(OracleError::NotInitialized);
        }

        provider.require_auth();

        if !storage::is_provider(&env, &provider) {
            return Err(OracleError::UnauthorizedProvider);
        }

        validation::validate_price(price)?;

        let mut feed = storage::get_feed(&env, &feed_id)
            .ok_or(OracleError::FeedNotFound)?;

        feed.price = price;
        feed.updated_at_ledger = env.ledger().sequence();
        feed.submitted_by = provider.clone();

        storage::save_feed(&env, &feed);
        events::emit_price_updated(&env, &feed_id, &provider, price);
        Ok(())
    }

    // ── Price retrieval ────────────────────────────────────────────────

    pub fn get_price(env: Env, feed_id: String) -> Result<PriceFeed, OracleError> {
        if !storage::is_initialized(&env) {
            return Err(OracleError::NotInitialized);
        }

        let feed = storage::get_feed(&env, &feed_id)
            .ok_or(OracleError::FeedNotFound)?;

        if feed.price == 0 {
            return Err(OracleError::FeedNotFound); // never populated
        }

        let config = storage::get_config(&env).ok_or(OracleError::NotInitialized)?;
        let current = env.ledger().sequence();
        let staleness = current.saturating_sub(feed.updated_at_ledger);

        if staleness > config.max_staleness_ledgers {
            return Err(OracleError::StalePriceFeed);
        }

        Ok(feed)
    }

    // ── Provider management ────────────────────────────────────────────

    pub fn add_provider(env: Env, admin: Address, provider: Address) -> Result<(), OracleError> {
        if !storage::is_initialized(&env) {
            return Err(OracleError::NotInitialized);
        }

        admin.require_auth();

        let config = storage::get_config(&env).ok_or(OracleError::NotInitialized)?;
        if admin != config.admin {
            return Err(OracleError::Unauthorized);
        }

        storage::add_provider(&env, &provider);
        events::emit_provider_added(&env, &provider, &admin);
        Ok(())
    }

    pub fn remove_provider(
        env: Env,
        admin: Address,
        provider: Address,
    ) -> Result<(), OracleError> {
        if !storage::is_initialized(&env) {
            return Err(OracleError::NotInitialized);
        }

        admin.require_auth();

        let config = storage::get_config(&env).ok_or(OracleError::NotInitialized)?;
        if admin != config.admin {
            return Err(OracleError::Unauthorized);
        }

        storage::remove_provider(&env, &provider);
        events::emit_provider_removed(&env, &provider, &admin);
        Ok(())
    }
}
