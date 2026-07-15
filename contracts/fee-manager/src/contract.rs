//! Fee-manager contract implementation.

use crate::errors::FeeManagerError;
use crate::events;
use crate::storage;
use crate::types::{DiscountRule, FeeBreakdown, FeeSchedule};
use crate::utils;
use crate::validation;
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct FeeManagerContract;

#[contractimpl]
impl FeeManagerContract {
    // ── Initialisation ─────────────────────────────────────────────────

    pub fn initialize(
        env: Env,
        admin: Address,
        schedule: FeeSchedule,
    ) -> Result<(), FeeManagerError> {
        if storage::is_initialized(&env) {
            return Err(FeeManagerError::AlreadyInitialized);
        }

        admin.require_auth();

        // Validate every rate in the schedule.
        validation::validate_fee_bps(schedule.platform_fee_bps)?;
        validation::validate_fee_bps(schedule.escrow_fee_bps)?;
        validation::validate_fee_bps(schedule.withdrawal_fee_bps)?;
        validation::validate_commission_bps(schedule.referral_commission_bps)?;

        storage::set_admin(&env, &admin);
        storage::set_fee_schedule(&env, &schedule);
        storage::set_initialized(&env);
        Ok(())
    }

    // ── Fee schedule management ────────────────────────────────────────

    pub fn update_fee_schedule(
        env: Env,
        admin: Address,
        schedule: FeeSchedule,
    ) -> Result<(), FeeManagerError> {
        if !storage::is_initialized(&env) {
            return Err(FeeManagerError::NotInitialized);
        }

        admin.require_auth();

        let stored_admin = storage::get_admin(&env).ok_or(FeeManagerError::NotInitialized)?;
        if admin != stored_admin {
            return Err(FeeManagerError::Unauthorized);
        }

        validation::validate_fee_bps(schedule.platform_fee_bps)?;
        validation::validate_fee_bps(schedule.escrow_fee_bps)?;
        validation::validate_fee_bps(schedule.withdrawal_fee_bps)?;
        validation::validate_commission_bps(schedule.referral_commission_bps)?;

        storage::set_fee_schedule(&env, &schedule);

        events::emit_fee_schedule_updated(
            &env,
            &admin,
            schedule.platform_fee_bps,
            schedule.escrow_fee_bps,
            schedule.withdrawal_fee_bps,
        );
        Ok(())
    }

    // ── Discount rules ─────────────────────────────────────────────────

    pub fn set_discount(
        env: Env,
        admin: Address,
        rule: DiscountRule,
    ) -> Result<(), FeeManagerError> {
        if !storage::is_initialized(&env) {
            return Err(FeeManagerError::NotInitialized);
        }

        admin.require_auth();

        let stored_admin = storage::get_admin(&env).ok_or(FeeManagerError::NotInitialized)?;
        if admin != stored_admin {
            return Err(FeeManagerError::Unauthorized);
        }

        validation::validate_discount_bps(rule.discount_bps)?;

        storage::set_discount(&env, &rule);

        events::emit_discount_set(
            &env,
            &rule.address,
            rule.discount_bps,
            rule.expires_at_ledger,
            &admin,
        );
        Ok(())
    }

    pub fn remove_discount(
        env: Env,
        admin: Address,
        address: Address,
    ) -> Result<(), FeeManagerError> {
        if !storage::is_initialized(&env) {
            return Err(FeeManagerError::NotInitialized);
        }

        admin.require_auth();

        let stored_admin = storage::get_admin(&env).ok_or(FeeManagerError::NotInitialized)?;
        if admin != stored_admin {
            return Err(FeeManagerError::Unauthorized);
        }

        if !storage::has_discount(&env, &address) {
            return Err(FeeManagerError::DiscountNotFound);
        }

        storage::remove_discount(&env, &address);
        events::emit_discount_removed(&env, &address, &admin);
        Ok(())
    }

    // ── Fee calculation (pure view functions) ──────────────────────────

    pub fn calculate_platform_fee(
        env: Env,
        principal: i128,
        payer: Address,
        referrer: Option<Address>,
    ) -> Result<FeeBreakdown, FeeManagerError> {
        if !storage::is_initialized(&env) {
            return Err(FeeManagerError::NotInitialized);
        }

        validation::validate_principal(principal)?;

        // Self-referral guard
        if let Some(ref r) = referrer {
            if r == &payer {
                return Err(FeeManagerError::SelfReferral);
            }
        }

        let schedule =
            storage::get_fee_schedule(&env).ok_or(FeeManagerError::NotInitialized)?;
        let discount = storage::get_discount(&env, &payer);

        utils::calculate_fee_breakdown(
            &env,
            principal,
            schedule.platform_fee_bps,
            &schedule,
            discount.as_ref(),
            referrer.is_some(),
        )
    }

    pub fn calculate_escrow_fee(
        env: Env,
        principal: i128,
        payer: Address,
        referrer: Option<Address>,
    ) -> Result<FeeBreakdown, FeeManagerError> {
        if !storage::is_initialized(&env) {
            return Err(FeeManagerError::NotInitialized);
        }

        validation::validate_principal(principal)?;

        if let Some(ref r) = referrer {
            if r == &payer {
                return Err(FeeManagerError::SelfReferral);
            }
        }

        let schedule =
            storage::get_fee_schedule(&env).ok_or(FeeManagerError::NotInitialized)?;
        let discount = storage::get_discount(&env, &payer);

        utils::calculate_fee_breakdown(
            &env,
            principal,
            schedule.escrow_fee_bps,
            &schedule,
            discount.as_ref(),
            referrer.is_some(),
        )
    }

    pub fn calculate_withdrawal_fee(
        env: Env,
        principal: i128,
        payer: Address,
        referrer: Option<Address>,
    ) -> Result<FeeBreakdown, FeeManagerError> {
        if !storage::is_initialized(&env) {
            return Err(FeeManagerError::NotInitialized);
        }

        validation::validate_principal(principal)?;

        if let Some(ref r) = referrer {
            if r == &payer {
                return Err(FeeManagerError::SelfReferral);
            }
        }

        let schedule =
            storage::get_fee_schedule(&env).ok_or(FeeManagerError::NotInitialized)?;
        let discount = storage::get_discount(&env, &payer);

        utils::calculate_fee_breakdown(
            &env,
            principal,
            schedule.withdrawal_fee_bps,
            &schedule,
            discount.as_ref(),
            referrer.is_some(),
        )
    }

    // ── View ───────────────────────────────────────────────────────────

    pub fn get_fee_schedule(env: Env) -> Result<FeeSchedule, FeeManagerError> {
        if !storage::is_initialized(&env) {
            return Err(FeeManagerError::NotInitialized);
        }
        storage::get_fee_schedule(&env).ok_or(FeeManagerError::NotInitialized)
    }

    pub fn get_discount(env: Env, address: Address) -> Option<DiscountRule> {
        storage::get_discount(&env, &address)
    }
}
