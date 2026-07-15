//! Public interface trait for the fee-manager contract.

use crate::errors::FeeManagerError;
use crate::types::{DiscountRule, FeeBreakdown, FeeSchedule};
use soroban_sdk::{Address, Env};

pub trait FeeManagerInterface {
    /// Initialise the fee-manager with a fee schedule.
    fn initialize(
        env: Env,
        admin: Address,
        schedule: FeeSchedule,
    ) -> Result<(), FeeManagerError>;

    /// Update the platform-wide fee schedule (admin only).
    fn update_fee_schedule(
        env: Env,
        admin: Address,
        schedule: FeeSchedule,
    ) -> Result<(), FeeManagerError>;

    /// Set a discount rule for a specific address (admin only).
    fn set_discount(
        env: Env,
        admin: Address,
        rule: DiscountRule,
    ) -> Result<(), FeeManagerError>;

    /// Remove a discount rule (admin only).
    fn remove_discount(
        env: Env,
        admin: Address,
        address: Address,
    ) -> Result<(), FeeManagerError>;

    /// Calculate the platform fee breakdown for a given principal.
    ///
    /// Pure computation — does not modify state or transfer tokens.
    fn calculate_platform_fee(
        env: Env,
        principal: i128,
        payer: Address,
        referrer: Option<Address>,
    ) -> Result<FeeBreakdown, FeeManagerError>;

    /// Calculate the escrow fee breakdown.
    fn calculate_escrow_fee(
        env: Env,
        principal: i128,
        payer: Address,
        referrer: Option<Address>,
    ) -> Result<FeeBreakdown, FeeManagerError>;

    /// Calculate the withdrawal fee breakdown.
    fn calculate_withdrawal_fee(
        env: Env,
        principal: i128,
        payer: Address,
        referrer: Option<Address>,
    ) -> Result<FeeBreakdown, FeeManagerError>;

    /// Return the current fee schedule.
    fn get_fee_schedule(env: Env) -> Result<FeeSchedule, FeeManagerError>;

    /// Return the discount rule for an address, if any.
    fn get_discount(env: Env, address: Address) -> Option<DiscountRule>;
}
