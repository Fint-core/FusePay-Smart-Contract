//! Escrow contract implementation.

use crate::errors::EscrowError;
use crate::events;
use crate::storage;
use crate::types::{CreateEscrowParams, Escrow, EscrowConfig, EscrowState};
use crate::utils;
use crate::validation;
use shared_utils::expiry_ledger;
use soroban_sdk::{contract, contractimpl, token, Address, Env};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    // ── Initialisation ─────────────────────────────────────────────────

    pub fn initialize(
        env: Env,
        admin: Address,
        treasury: Address,
        fee_bps: i128,
    ) -> Result<(), EscrowError> {
        if storage::is_initialized(&env) {
            return Err(EscrowError::AlreadyInitialized);
        }

        admin.require_auth();

        if fee_bps < 0 {
            return Err(EscrowError::InvalidAmount);
        }

        let config = EscrowConfig {
            admin,
            treasury,
            fee_bps,
        };

        storage::set_config(&env, &config);
        storage::set_initialized(&env);
        Ok(())
    }

    // ── Create ─────────────────────────────────────────────────────────

    /// Create a new escrow. Returns the escrow ID.
    pub fn create_escrow(env: Env, params: CreateEscrowParams) -> Result<u64, EscrowError> {
        if !storage::is_initialized(&env) {
            return Err(EscrowError::NotInitialized);
        }

        params.depositor.require_auth();

        validation::validate_amount(params.amount)?;
        validation::validate_parties(&params.depositor, &params.beneficiary)?;
        let timeout = validation::validate_timeout(params.timeout_ledgers)?;

        let config = storage::get_config(&env).ok_or(EscrowError::NotInitialized)?;
        let (net_amount, fee_amount) =
            utils::compute_escrow_amounts(params.amount, config.fee_bps)?;

        let escrow_id = storage::next_escrow_id(&env);
        let expires_at =
            expiry_ledger(&env, timeout).map_err(|_| EscrowError::ArithmeticOverflow)?;

        let escrow = Escrow {
            escrow_id,
            depositor: params.depositor.clone(),
            beneficiary: params.beneficiary.clone(),
            arbiter: params.arbiter,
            token: params.token.clone(),
            amount: params.amount,
            fee_amount,
            net_amount,
            state: EscrowState::Pending,
            expires_at_ledger: expires_at,
            created_at_ledger: env.ledger().sequence(),
            metadata: params.metadata,
        };

        storage::save_escrow(&env, &escrow);
        events::emit_escrow_created(
            &env,
            escrow_id,
            &params.depositor,
            &params.beneficiary,
            &params.token,
            params.amount,
            expires_at,
        );

        Ok(escrow_id)
    }

    // ── Fund ───────────────────────────────────────────────────────────

    /// Deposit funds into a `Pending` escrow to move it to `Funded`.
    pub fn fund_escrow(env: Env, depositor: Address, escrow_id: u64) -> Result<(), EscrowError> {
        if !storage::is_initialized(&env) {
            return Err(EscrowError::NotInitialized);
        }

        depositor.require_auth();

        let mut escrow = storage::get_escrow(&env, escrow_id)
            .ok_or(EscrowError::EscrowNotFound)?;

        if escrow.depositor != depositor {
            return Err(EscrowError::Unauthorized);
        }

        validation::require_state(&escrow.state, &EscrowState::Pending)?;
        validation::require_not_expired(&env, escrow.expires_at_ledger)?;

        // Transfer the full amount from depositor to this contract.
        let token_client = token::Client::new(&env, &escrow.token);
        token_client.transfer(
            &depositor,
            &env.current_contract_address(),
            &escrow.amount,
        );

        escrow.state = EscrowState::Funded;
        storage::save_escrow(&env, &escrow);

        events::emit_escrow_funded(&env, escrow_id, &depositor, escrow.amount);
        Ok(())
    }

    // ── Release ────────────────────────────────────────────────────────

    /// Release funds to the beneficiary. Called by depositor or arbiter.
    pub fn release_escrow(env: Env, caller: Address, escrow_id: u64) -> Result<(), EscrowError> {
        if !storage::is_initialized(&env) {
            return Err(EscrowError::NotInitialized);
        }

        caller.require_auth();

        let mut escrow = storage::get_escrow(&env, escrow_id)
            .ok_or(EscrowError::EscrowNotFound)?;

        // Only the depositor or arbiter can release.
        if caller != escrow.depositor && caller != escrow.arbiter {
            return Err(EscrowError::Unauthorized);
        }

        validation::require_state(&escrow.state, &EscrowState::Funded)?;
        validation::require_not_expired(&env, escrow.expires_at_ledger)?;

        let config = storage::get_config(&env).ok_or(EscrowError::NotInitialized)?;
        let token_client = token::Client::new(&env, &escrow.token);

        // Update state before transfers (CEI pattern).
        escrow.state = EscrowState::Released;
        storage::save_escrow(&env, &escrow);

        // Transfer net amount to beneficiary.
        token_client.transfer(
            &env.current_contract_address(),
            &escrow.beneficiary,
            &escrow.net_amount,
        );

        // Transfer fee to treasury (if non-zero).
        if escrow.fee_amount > 0 {
            token_client.transfer(
                &env.current_contract_address(),
                &config.treasury,
                &escrow.fee_amount,
            );
        }

        events::emit_escrow_released(
            &env,
            escrow_id,
            &escrow.beneficiary,
            escrow.net_amount,
            escrow.fee_amount,
            &caller,
        );
        Ok(())
    }

    // ── Refund ─────────────────────────────────────────────────────────

    /// Refund the depositor. Called by beneficiary or arbiter.
    pub fn refund_escrow(env: Env, caller: Address, escrow_id: u64) -> Result<(), EscrowError> {
        if !storage::is_initialized(&env) {
            return Err(EscrowError::NotInitialized);
        }

        caller.require_auth();

        let mut escrow = storage::get_escrow(&env, escrow_id)
            .ok_or(EscrowError::EscrowNotFound)?;

        // Beneficiary or arbiter can refund.
        if caller != escrow.beneficiary && caller != escrow.arbiter {
            return Err(EscrowError::Unauthorized);
        }

        validation::require_state(&escrow.state, &EscrowState::Funded)?;

        let token_client = token::Client::new(&env, &escrow.token);

        // Update state before transfer.
        escrow.state = EscrowState::Refunded;
        storage::save_escrow(&env, &escrow);

        token_client.transfer(
            &env.current_contract_address(),
            &escrow.depositor,
            &escrow.amount,
        );

        events::emit_escrow_refunded(&env, escrow_id, &escrow.depositor, escrow.amount, &caller);
        Ok(())
    }

    // ── Dispute ────────────────────────────────────────────────────────

    /// Raise a dispute on a `Funded` escrow.
    pub fn raise_dispute(env: Env, caller: Address, escrow_id: u64) -> Result<(), EscrowError> {
        if !storage::is_initialized(&env) {
            return Err(EscrowError::NotInitialized);
        }

        caller.require_auth();

        let mut escrow = storage::get_escrow(&env, escrow_id)
            .ok_or(EscrowError::EscrowNotFound)?;

        // Either party can raise a dispute.
        if caller != escrow.depositor && caller != escrow.beneficiary {
            return Err(EscrowError::Unauthorized);
        }

        validation::require_state(&escrow.state, &EscrowState::Funded)?;

        escrow.state = EscrowState::Disputed;
        storage::save_escrow(&env, &escrow);

        events::emit_dispute_raised(&env, escrow_id, &caller);
        Ok(())
    }

    /// Resolve a `Disputed` escrow. Only the arbiter can call this.
    pub fn resolve_dispute(
        env: Env,
        arbiter: Address,
        escrow_id: u64,
        release: bool,
    ) -> Result<(), EscrowError> {
        if !storage::is_initialized(&env) {
            return Err(EscrowError::NotInitialized);
        }

        arbiter.require_auth();

        let mut escrow = storage::get_escrow(&env, escrow_id)
            .ok_or(EscrowError::EscrowNotFound)?;

        if arbiter != escrow.arbiter {
            return Err(EscrowError::Unauthorized);
        }

        validation::require_state(&escrow.state, &EscrowState::Disputed)?;

        let config = storage::get_config(&env).ok_or(EscrowError::NotInitialized)?;
        let token_client = token::Client::new(&env, &escrow.token);

        if release {
            escrow.state = EscrowState::Released;
            storage::save_escrow(&env, &escrow);

            token_client.transfer(
                &env.current_contract_address(),
                &escrow.beneficiary,
                &escrow.net_amount,
            );

            if escrow.fee_amount > 0 {
                token_client.transfer(
                    &env.current_contract_address(),
                    &config.treasury,
                    &escrow.fee_amount,
                );
            }

            events::emit_escrow_released(
                &env,
                escrow_id,
                &escrow.beneficiary,
                escrow.net_amount,
                escrow.fee_amount,
                &arbiter,
            );
        } else {
            escrow.state = EscrowState::Refunded;
            storage::save_escrow(&env, &escrow);

            token_client.transfer(
                &env.current_contract_address(),
                &escrow.depositor,
                &escrow.amount,
            );

            events::emit_escrow_refunded(
                &env,
                escrow_id,
                &escrow.depositor,
                escrow.amount,
                &arbiter,
            );
        }

        Ok(())
    }

    // ── Cancel ─────────────────────────────────────────────────────────

    /// Cancel a `Pending` escrow before funding.
    pub fn cancel_escrow(
        env: Env,
        depositor: Address,
        escrow_id: u64,
    ) -> Result<(), EscrowError> {
        if !storage::is_initialized(&env) {
            return Err(EscrowError::NotInitialized);
        }

        depositor.require_auth();

        let mut escrow = storage::get_escrow(&env, escrow_id)
            .ok_or(EscrowError::EscrowNotFound)?;

        if depositor != escrow.depositor {
            return Err(EscrowError::Unauthorized);
        }

        validation::require_state(&escrow.state, &EscrowState::Pending)?;

        escrow.state = EscrowState::Cancelled;
        storage::save_escrow(&env, &escrow);

        events::emit_escrow_cancelled(&env, escrow_id, &depositor);
        Ok(())
    }

    // ── Timeout refund ─────────────────────────────────────────────────

    /// Refund a `Funded` escrow that has passed its expiry ledger.
    pub fn claim_timeout_refund(
        env: Env,
        caller: Address,
        escrow_id: u64,
    ) -> Result<(), EscrowError> {
        if !storage::is_initialized(&env) {
            return Err(EscrowError::NotInitialized);
        }

        caller.require_auth();

        let mut escrow = storage::get_escrow(&env, escrow_id)
            .ok_or(EscrowError::EscrowNotFound)?;

        // Only the depositor can claim a timeout refund.
        if caller != escrow.depositor {
            return Err(EscrowError::Unauthorized);
        }

        validation::require_state(&escrow.state, &EscrowState::Funded)?;
        validation::require_expired(&env, escrow.expires_at_ledger)?;

        let token_client = token::Client::new(&env, &escrow.token);

        escrow.state = EscrowState::Expired;
        storage::save_escrow(&env, &escrow);

        token_client.transfer(
            &env.current_contract_address(),
            &escrow.depositor,
            &escrow.amount,
        );

        events::emit_escrow_expired(&env, escrow_id, &escrow.depositor, escrow.amount);
        Ok(())
    }

    // ── Admin ──────────────────────────────────────────────────────────

    /// Update the escrow fee rate. Admin only.
    pub fn update_fee(env: Env, admin: Address, new_fee_bps: i128) -> Result<(), EscrowError> {
        if !storage::is_initialized(&env) {
            return Err(EscrowError::NotInitialized);
        }

        admin.require_auth();

        let mut config = storage::get_config(&env).ok_or(EscrowError::NotInitialized)?;

        if admin != config.admin {
            return Err(EscrowError::Unauthorized);
        }

        if new_fee_bps < 0 {
            return Err(EscrowError::InvalidAmount);
        }

        config.fee_bps = new_fee_bps;
        storage::set_config(&env, &config);
        Ok(())
    }

    // ── View ───────────────────────────────────────────────────────────

    /// Retrieve an escrow record by ID.
    pub fn get_escrow(env: Env, escrow_id: u64) -> Result<Escrow, EscrowError> {
        storage::get_escrow(&env, escrow_id).ok_or(EscrowError::EscrowNotFound)
    }
}
