//! Treasury contract implementation.

use crate::errors::TreasuryError;
use crate::events;
use crate::storage;
use crate::types::{DistributionEntry, TokenLedger, TreasuryConfig};
use crate::utils;
use crate::validation;
use soroban_sdk::{contract, contractimpl, token, Address, Env, Vec};

#[contract]
pub struct TreasuryContract;

#[contractimpl]
impl TreasuryContract {
    // ── Initialisation ─────────────────────────────────────────────────

    pub fn initialize(
        env: Env,
        admin: Address,
        multisig_contract: Address,
        multisig_threshold: i128,
    ) -> Result<(), TreasuryError> {
        if storage::is_initialized(&env) {
            return Err(TreasuryError::AlreadyInitialized);
        }

        admin.require_auth();

        if multisig_threshold <= 0 {
            return Err(TreasuryError::InvalidAmount);
        }

        let config = TreasuryConfig {
            admin,
            multisig_contract,
            multisig_threshold,
        };

        storage::set_config(&env, &config);
        storage::set_initialized(&env);
        Ok(())
    }

    // ── Deposit ────────────────────────────────────────────────────────

    /// Accept a token deposit from `sender` into the treasury.
    pub fn deposit(
        env: Env,
        sender: Address,
        token: Address,
        amount: i128,
    ) -> Result<(), TreasuryError> {
        if !storage::is_initialized(&env) {
            return Err(TreasuryError::NotInitialized);
        }

        sender.require_auth();
        validation::validate_amount(amount)?;

        // Pull tokens from sender.
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&sender, &env.current_contract_address(), &amount);

        // Update accounting.
        let mut ledger = storage::get_or_create_token_ledger(&env, &token);
        ledger.total_deposited = utils::add_balance(ledger.total_deposited, amount)?;
        ledger.available = utils::add_balance(ledger.available, amount)?;
        storage::save_token_ledger(&env, &ledger);

        events::emit_treasury_deposit(&env, &token, amount, &sender, ledger.available);
        Ok(())
    }

    // ── Withdraw ───────────────────────────────────────────────────────

    /// Withdraw from the treasury. For amounts >= multisig_threshold, the
    /// admin must have obtained multisig approval beforehand (enforced
    /// off-chain / by governance flow; this contract enforces the limit check).
    pub fn withdraw(
        env: Env,
        admin: Address,
        token: Address,
        amount: i128,
        recipient: Address,
    ) -> Result<(), TreasuryError> {
        if !storage::is_initialized(&env) {
            return Err(TreasuryError::NotInitialized);
        }

        admin.require_auth();

        let config = storage::get_config(&env).ok_or(TreasuryError::NotInitialized)?;
        if admin != config.admin {
            return Err(TreasuryError::Unauthorized);
        }

        validation::validate_amount(amount)?;
        validation::validate_single_signer_limit(amount, config.multisig_threshold)?;

        let mut ledger = storage::get_or_create_token_ledger(&env, &token);
        validation::validate_sufficient_balance(ledger.available, amount)?;

        // Update accounting before external transfer (CEI).
        ledger.total_distributed = utils::add_balance(ledger.total_distributed, amount)?;
        ledger.available = utils::sub_balance(ledger.available, amount)?;
        storage::save_token_ledger(&env, &ledger);

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&env.current_contract_address(), &recipient, &amount);

        events::emit_treasury_withdraw(
            &env,
            &token,
            amount,
            &recipient,
            &admin,
            ledger.available,
        );
        Ok(())
    }

    // ── Distribute ─────────────────────────────────────────────────────

    /// Batch distribution to multiple recipients. Also subject to the
    /// single-signer limit on the total sum.
    pub fn distribute(
        env: Env,
        admin: Address,
        token: Address,
        entries: Vec<DistributionEntry>,
    ) -> Result<(), TreasuryError> {
        if !storage::is_initialized(&env) {
            return Err(TreasuryError::NotInitialized);
        }

        admin.require_auth();

        let config = storage::get_config(&env).ok_or(TreasuryError::NotInitialized)?;
        if admin != config.admin {
            return Err(TreasuryError::Unauthorized);
        }

        if entries.is_empty() {
            return Err(TreasuryError::InvalidDistribution);
        }

        let total = utils::sum_distribution(&entries)?;
        validation::validate_amount(total)?;
        validation::validate_single_signer_limit(total, config.multisig_threshold)?;

        let mut ledger = storage::get_or_create_token_ledger(&env, &token);
        validation::validate_sufficient_balance(ledger.available, total)?;

        // Update accounting before any transfers (CEI).
        ledger.total_distributed = utils::add_balance(ledger.total_distributed, total)?;
        ledger.available = utils::sub_balance(ledger.available, total)?;
        storage::save_token_ledger(&env, &ledger);

        let token_client = token::Client::new(&env, &token);
        let contract_addr = env.current_contract_address();

        for entry in entries.iter() {
            validation::validate_amount(entry.amount)?;
            token_client.transfer(&contract_addr, &entry.recipient, &entry.amount);
        }

        events::emit_distribution(&env, &token, total, entries.len(), &admin);
        Ok(())
    }

    // ── Admin ──────────────────────────────────────────────────────────

    pub fn update_multisig_threshold(
        env: Env,
        admin: Address,
        new_threshold: i128,
    ) -> Result<(), TreasuryError> {
        if !storage::is_initialized(&env) {
            return Err(TreasuryError::NotInitialized);
        }

        admin.require_auth();

        let mut config = storage::get_config(&env).ok_or(TreasuryError::NotInitialized)?;
        if admin != config.admin {
            return Err(TreasuryError::Unauthorized);
        }

        if new_threshold <= 0 {
            return Err(TreasuryError::InvalidAmount);
        }

        config.multisig_threshold = new_threshold;
        storage::set_config(&env, &config);
        Ok(())
    }

    // ── View ───────────────────────────────────────────────────────────

    pub fn get_token_ledger(env: Env, token: Address) -> Result<TokenLedger, TreasuryError> {
        if !storage::is_initialized(&env) {
            return Err(TreasuryError::NotInitialized);
        }
        Ok(storage::get_or_create_token_ledger(&env, &token))
    }

    pub fn get_config(env: Env) -> Result<TreasuryConfig, TreasuryError> {
        if !storage::is_initialized(&env) {
            return Err(TreasuryError::NotInitialized);
        }
        storage::get_config(&env).ok_or(TreasuryError::NotInitialized)
    }
}
