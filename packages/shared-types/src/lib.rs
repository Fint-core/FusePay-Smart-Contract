//! # shared-types
//!
//! Common types shared across all FusePay smart contracts.
//!
//! These types represent the core domain model for the FusePay platform.
//! Every contract that needs to pass wallet addresses, token amounts, or
//! role identifiers should import from this crate rather than re-defining
//! its own types.

#![no_std]

use soroban_sdk::{contracttype, Address, String};

// ── Token amount ───────────────────────────────────────────────────────────

/// A signed 128-bit integer used for all token amounts.
/// Stellar Soroban tokens use i128 as the canonical amount type.
/// Negative values are never valid in FusePay; validation must reject them.
pub type TokenAmount = i128;

// ── Roles ─────────────────────────────────────────────────────────────────

/// Role identifiers used by the access-control contract.
///
/// Roles are ordered from highest to lowest privilege. Every privilege check
/// should reference these variants rather than raw strings to prevent typos.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Role {
    /// Superuser — can transfer ownership, set admins, and upgrade contracts.
    Owner,
    /// Administrative role — can configure contract parameters.
    Admin,
    /// Operational role — can trigger day-to-day operations (e.g. fee collection).
    Operator,
    /// Treasury role — can authorise treasury withdrawals below threshold.
    Treasury,
    /// Read-only audit role — cannot modify state.
    Auditor,
}

// ── Escrow state ───────────────────────────────────────────────────────────

/// Lifecycle state of an escrow agreement.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EscrowState {
    /// Escrow has been created but no funds have been deposited yet.
    Pending,
    /// Funds have been deposited and are locked.
    Funded,
    /// Funds have been released to the seller.
    Released,
    /// Funds have been refunded to the buyer.
    Refunded,
    /// A dispute has been raised and is awaiting resolution.
    Disputed,
    /// Escrow was cancelled before funding.
    Cancelled,
    /// Escrow expired without resolution and was refunded automatically.
    Expired,
}

// ── Multisig proposal state ────────────────────────────────────────────────

/// Lifecycle state of a multisig proposal.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProposalState {
    /// Proposal is open and collecting approvals.
    Active,
    /// Proposal has reached the required threshold and has been executed.
    Executed,
    /// Proposal was explicitly rejected by sufficient owners.
    Rejected,
    /// Proposal expired without reaching threshold.
    Expired,
}

// ── Wallet info ────────────────────────────────────────────────────────────

/// Summary information returned by wallet query functions.
#[contracttype]
#[derive(Clone, Debug)]
pub struct WalletInfo {
    /// The wallet owner's Stellar address.
    pub owner: Address,
    /// The token contract address this wallet operates with.
    pub token: Address,
    /// Current balance in stroops.
    pub balance: TokenAmount,
    /// Whether the wallet is currently active (not frozen).
    pub is_active: bool,
    /// Ledger sequence number when the wallet was created.
    pub created_at_ledger: u32,
}

// ── Escrow record ──────────────────────────────────────────────────────────

/// Full escrow record stored on-chain.
#[contracttype]
#[derive(Clone, Debug)]
pub struct EscrowRecord {
    /// Unique identifier for this escrow.
    pub escrow_id: u64,
    /// The party depositing funds.
    pub depositor: Address,
    /// The party receiving funds upon release.
    pub beneficiary: Address,
    /// Neutral arbiter who can resolve disputes (may equal a platform address).
    pub arbiter: Address,
    /// Token contract address.
    pub token: Address,
    /// Total amount locked in this escrow.
    pub amount: TokenAmount,
    /// Platform fee deducted on release.
    pub fee_amount: TokenAmount,
    /// Current lifecycle state.
    pub state: EscrowState,
    /// Ledger sequence after which the escrow can be refunded automatically.
    pub expires_at_ledger: u32,
    /// Ledger sequence when this escrow was created.
    pub created_at_ledger: u32,
    /// Optional metadata (e.g. order reference, up to 64 bytes).
    pub metadata: String,
}

// ── Multisig proposal ──────────────────────────────────────────────────────

/// A multisig proposal waiting for owner approvals.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Proposal {
    /// Unique sequential ID.
    pub proposal_id: u64,
    /// Human-readable description of what this proposal does.
    pub description: String,
    /// Address of the contract to call upon execution.
    pub target_contract: Address,
    /// ABI-encoded call data (function selector + arguments).
    /// Encoded as bytes; the calling contract is responsible for decoding.
    pub call_data: soroban_sdk::Bytes,
    /// Current approval count.
    pub approval_count: u32,
    /// Current rejection count.
    pub rejection_count: u32,
    /// Current state.
    pub state: ProposalState,
    /// Ledger after which the proposal expires.
    pub expires_at_ledger: u32,
    /// Ledger when the proposal was created.
    pub created_at_ledger: u32,
    /// Address of the owner who submitted this proposal.
    pub proposer: Address,
}

// ── Fee configuration ──────────────────────────────────────────────────────

/// Fee configuration snapshot stored in fee-manager.
#[contracttype]
#[derive(Clone, Debug)]
pub struct FeeConfig {
    /// Platform fee in basis points.
    pub platform_fee_bps: i128,
    /// Escrow service fee in basis points.
    pub escrow_fee_bps: i128,
    /// Withdrawal fee in basis points.
    pub withdrawal_fee_bps: i128,
    /// Referral commission in basis points.
    pub referral_commission_bps: i128,
    /// Address that collected fees are sent to (treasury contract).
    pub fee_recipient: Address,
}

// ── Treasury record ────────────────────────────────────────────────────────

/// Treasury balance snapshot.
#[contracttype]
#[derive(Clone, Debug)]
pub struct TreasuryBalance {
    /// Token contract address.
    pub token: Address,
    /// Total amount collected (running total, never decremented).
    pub total_collected: TokenAmount,
    /// Amount that has been distributed / withdrawn.
    pub total_distributed: TokenAmount,
    /// Current available balance (collected - distributed).
    pub available: TokenAmount,
}
