//! Wallet contract implementation.

use crate::errors::WalletError;
use crate::events;
use crate::storage;
use crate::types::Wallet;
use crate::utils;
use crate::validation;
use soroban_sdk::{contract, contractimpl, token, Address, Env};

#[contract]
pub struct WalletContract;

#[contractimpl]
impl WalletContract {
    // ── Initialisation ─────────────────────────────────────────────────

    /// Initialise the wallet contract with a platform admin address.
    pub fn initialize(env: Env, admin: Address) -> Result<(), WalletError> {
        if storage::is_initialized(&env) {
            return Err(WalletError::AlreadyInitialized);
        }

        admin.require_auth();

        storage::set_admin(&env, &admin);
        storage::set_initialized(&env);
        Ok(())
    }

    // ── Wallet lifecycle ────────────────────────────────────────────────

    /// Create a new wallet for `owner` using `token`.
    pub fn create_wallet(env: Env, owner: Address, token: Address) -> Result<(), WalletError> {
        if !storage::is_initialized(&env) {
            return Err(WalletError::NotInitialized);
        }

        owner.require_auth();

        if storage::wallet_exists(&env, &owner, &token) {
            return Err(WalletError::WalletAlreadyExists);
        }

        let wallet = Wallet {
            owner: owner.clone(),
            token: token.clone(),
            balance: 0,
            is_active: true,
            created_at_ledger: env.ledger().sequence(),
            last_tx_ledger: env.ledger().sequence(),
            total_deposited: 0,
            total_withdrawn: 0,
        };

        storage::save_wallet(&env, &wallet);
        events::emit_wallet_created(&env, &owner, &token);
        Ok(())
    }

    // ── Deposit ────────────────────────────────────────────────────────

    /// Deposit `amount` of `token` into the wallet owned by `owner`.
    pub fn deposit(
        env: Env,
        owner: Address,
        token: Address,
        amount: i128,
    ) -> Result<(), WalletError> {
        if !storage::is_initialized(&env) {
            return Err(WalletError::NotInitialized);
        }

        owner.require_auth();

        validation::validate_amount(amount)?;

        let mut wallet = storage::get_wallet(&env, &owner, &token)
            .ok_or(WalletError::NotInitialized)?;

        validation::validate_token(&token, &wallet.token)?;
        validation::validate_wallet_active(wallet.is_active)?;

        // Transfer tokens from caller to this contract.
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&owner, &env.current_contract_address(), &amount);

        // Update wallet state using safe arithmetic.
        let new_balance = utils::add_to_balance(wallet.balance, amount)?;
        let new_total_deposited =
            utils::add_to_balance(wallet.total_deposited, amount)?;

        wallet.balance = new_balance;
        wallet.total_deposited = new_total_deposited;
        wallet.last_tx_ledger = env.ledger().sequence();

        storage::save_wallet(&env, &wallet);
        events::emit_deposit(&env, &owner, &token, amount, new_balance);
        Ok(())
    }

    // ── Withdrawal ─────────────────────────────────────────────────────

    /// Withdraw `amount` of `token` from the wallet owned by `owner`.
    pub fn withdraw(
        env: Env,
        owner: Address,
        token: Address,
        amount: i128,
        recipient: Address,
    ) -> Result<(), WalletError> {
        if !storage::is_initialized(&env) {
            return Err(WalletError::NotInitialized);
        }

        owner.require_auth();

        validation::validate_amount(amount)?;

        let mut wallet = storage::get_wallet(&env, &owner, &token)
            .ok_or(WalletError::NotInitialized)?;

        validation::validate_token(&token, &wallet.token)?;
        validation::validate_wallet_active(wallet.is_active)?;
        validation::validate_sufficient_balance(wallet.balance, amount)?;

        // Update wallet state before external call (checks-effects-interactions).
        let new_balance = utils::subtract_from_balance(wallet.balance, amount)?;
        let new_total_withdrawn =
            utils::add_to_balance(wallet.total_withdrawn, amount)?;

        wallet.balance = new_balance;
        wallet.total_withdrawn = new_total_withdrawn;
        wallet.last_tx_ledger = env.ledger().sequence();

        storage::save_wallet(&env, &wallet);

        // Transfer tokens to recipient after state is updated.
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&env.current_contract_address(), &recipient, &amount);

        events::emit_withdrawal(&env, &owner, &token, amount, &recipient, new_balance);
        Ok(())
    }

    // ── Admin operations ────────────────────────────────────────────────

    /// Freeze a wallet. Only the platform admin can call this.
    pub fn freeze_wallet(
        env: Env,
        admin: Address,
        owner: Address,
        token: Address,
    ) -> Result<(), WalletError> {
        if !storage::is_initialized(&env) {
            return Err(WalletError::NotInitialized);
        }

        admin.require_auth();

        let stored_admin = storage::get_admin(&env).ok_or(WalletError::NotInitialized)?;
        if admin != stored_admin {
            return Err(WalletError::Unauthorized);
        }

        let mut wallet = storage::get_wallet(&env, &owner, &token)
            .ok_or(WalletError::NotInitialized)?;

        wallet.is_active = false;
        storage::save_wallet(&env, &wallet);

        events::emit_wallet_frozen(&env, &owner, &admin);
        Ok(())
    }

    /// Unfreeze a wallet. Only the platform admin can call this.
    pub fn unfreeze_wallet(
        env: Env,
        admin: Address,
        owner: Address,
        token: Address,
    ) -> Result<(), WalletError> {
        if !storage::is_initialized(&env) {
            return Err(WalletError::NotInitialized);
        }

        admin.require_auth();

        let stored_admin = storage::get_admin(&env).ok_or(WalletError::NotInitialized)?;
        if admin != stored_admin {
            return Err(WalletError::Unauthorized);
        }

        let mut wallet = storage::get_wallet(&env, &owner, &token)
            .ok_or(WalletError::NotInitialized)?;

        wallet.is_active = true;
        storage::save_wallet(&env, &wallet);

        events::emit_wallet_unfrozen(&env, &owner, &admin);
        Ok(())
    }

    // ── View functions ──────────────────────────────────────────────────

    /// Returns the current balance of a wallet.
    pub fn get_balance(env: Env, owner: Address, token: Address) -> Result<i128, WalletError> {
        let wallet = storage::get_wallet(&env, &owner, &token)
            .ok_or(WalletError::NotInitialized)?;
        Ok(wallet.balance)
    }

    /// Returns the full wallet record.
    pub fn get_wallet(env: Env, owner: Address, token: Address) -> Result<Wallet, WalletError> {
        storage::get_wallet(&env, &owner, &token).ok_or(WalletError::NotInitialized)
    }
}
