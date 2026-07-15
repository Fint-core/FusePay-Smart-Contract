//! # shared-constants
//!
//! Compile-time constants shared across all FusePay smart contracts.
//!
//! All numeric limits, fee basis points, time windows, and storage namespace
//! prefixes are defined here so they can be updated in a single location and
//! referenced uniformly by every contract in the workspace.

#![no_std]

// ── Fee constants ──────────────────────────────────────────────────────────

/// Denominator used for basis-point fee calculations.
/// A fee of 25 bp = 25 / FEE_DENOMINATOR = 0.25 %.
pub const FEE_DENOMINATOR: i128 = 10_000;

/// Maximum platform fee in basis points (5 %).
pub const MAX_PLATFORM_FEE_BPS: i128 = 500;

/// Default platform fee in basis points (0.5 %).
pub const DEFAULT_PLATFORM_FEE_BPS: i128 = 50;

/// Default escrow service fee in basis points (1 %).
pub const DEFAULT_ESCROW_FEE_BPS: i128 = 100;

/// Default withdrawal fee in basis points (0.25 %).
pub const DEFAULT_WITHDRAWAL_FEE_BPS: i128 = 25;

/// Maximum referral commission in basis points (2 %).
pub const MAX_REFERRAL_COMMISSION_BPS: i128 = 200;

/// Default referral commission in basis points (0.5 %).
pub const DEFAULT_REFERRAL_COMMISSION_BPS: i128 = 50;

// ── Amount limits ──────────────────────────────────────────────────────────

/// Minimum deposit amount in stroops (0.0000001 XLM = 1 stroop).
pub const MIN_DEPOSIT_AMOUNT: i128 = 1;

/// Maximum single transaction amount in stroops (1,000,000 XLM).
pub const MAX_TRANSACTION_AMOUNT: i128 = 1_000_000 * 10_000_000;

/// Minimum escrow amount in stroops (1 XLM).
pub const MIN_ESCROW_AMOUNT: i128 = 10_000_000;

// ── Time / ledger constants ────────────────────────────────────────────────

/// Default escrow timeout in ledgers.
/// Stellar produces roughly one ledger every 5 seconds.
/// 17,280 ledgers ≈ 24 hours.
pub const DEFAULT_ESCROW_TIMEOUT_LEDGERS: u32 = 17_280;

/// Maximum escrow timeout in ledgers (30 days ≈ 518,400 ledgers).
pub const MAX_ESCROW_TIMEOUT_LEDGERS: u32 = 518_400;

/// Default multisig proposal expiry in ledgers (48 hours ≈ 34,560 ledgers).
pub const DEFAULT_MULTISIG_EXPIRY_LEDGERS: u32 = 34_560;

/// Maximum multisig proposal expiry in ledgers (7 days ≈ 120,960 ledgers).
pub const MAX_MULTISIG_EXPIRY_LEDGERS: u32 = 120_960;

// ── Multisig limits ────────────────────────────────────────────────────────

/// Maximum number of owners in a multisig configuration.
pub const MAX_MULTISIG_OWNERS: u32 = 20;

/// Minimum multisig threshold (at least 1 approval required).
pub const MIN_MULTISIG_THRESHOLD: u32 = 1;

// ── Storage TTL (ledgers) ──────────────────────────────────────────────────

/// Persistent storage TTL extension bump — 1 year worth of ledgers.
/// Used when bumping the TTL on critical persistent entries.
pub const PERSISTENT_BUMP_LEDGERS: u32 = 6_307_200; // ~1 year at 5s/ledger

/// Instance storage TTL — 30 days.
pub const INSTANCE_BUMP_LEDGERS: u32 = 518_400;

/// Temporary storage TTL — 1 day.
pub const TEMPORARY_BUMP_LEDGERS: u32 = 17_280;

// ── Storage namespace prefixes ─────────────────────────────────────────────

/// Prefix for wallet storage keys.
pub const STORAGE_PREFIX_WALLET: &str = "FP_WALLET";

/// Prefix for escrow storage keys.
pub const STORAGE_PREFIX_ESCROW: &str = "FP_ESCROW";

/// Prefix for multisig storage keys.
pub const STORAGE_PREFIX_MULTISIG: &str = "FP_MULTISIG";

/// Prefix for treasury storage keys.
pub const STORAGE_PREFIX_TREASURY: &str = "FP_TREASURY";

/// Prefix for access-control storage keys.
pub const STORAGE_PREFIX_ACCESS: &str = "FP_ACCESS";

/// Prefix for fee-manager storage keys.
pub const STORAGE_PREFIX_FEE: &str = "FP_FEE";

/// Prefix for oracle storage keys.
pub const STORAGE_PREFIX_ORACLE: &str = "FP_ORACLE";

// ── Version ────────────────────────────────────────────────────────────────

/// Current contract interface version.
/// Increment this when the public API changes in a breaking way.
pub const CONTRACT_VERSION: u32 = 1;
