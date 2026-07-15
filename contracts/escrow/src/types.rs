//! Types specific to the escrow contract.

use soroban_sdk::{contracttype, Address, String};

pub use shared_types::EscrowState;

/// Full escrow record stored on-chain.
///
/// This mirrors `shared_types::EscrowRecord` but lives in the escrow
/// contract crate so the contract can own and extend the definition.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Escrow {
    /// Auto-incremented unique identifier.
    pub escrow_id: u64,
    /// The party depositing funds (buyer in a marketplace context).
    pub depositor: Address,
    /// The party receiving funds upon a successful release (seller).
    pub beneficiary: Address,
    /// Neutral party who can resolve disputes. May be the platform address.
    pub arbiter: Address,
    /// Soroban token contract address.
    pub token: Address,
    /// Total locked amount (inclusive of fee).
    pub amount: i128,
    /// Fee amount that will be sent to the treasury on release.
    pub fee_amount: i128,
    /// Net amount that will be sent to the beneficiary on release.
    pub net_amount: i128,
    /// Current lifecycle state.
    pub state: EscrowState,
    /// Ledger after which this escrow can be refunded automatically.
    pub expires_at_ledger: u32,
    /// Ledger when this escrow was created.
    pub created_at_ledger: u32,
    /// Optional short reference string (order ID, invoice number, etc.).
    pub metadata: String,
}

/// Parameters required to create a new escrow.
#[contracttype]
#[derive(Clone, Debug)]
pub struct CreateEscrowParams {
    pub depositor: Address,
    pub beneficiary: Address,
    pub arbiter: Address,
    pub token: Address,
    pub amount: i128,
    /// Timeout in ledgers from now. Use 0 for default.
    pub timeout_ledgers: u32,
    pub metadata: String,
}

/// Configuration stored at contract initialisation.
#[contracttype]
#[derive(Clone, Debug)]
pub struct EscrowConfig {
    /// Platform admin that controls configuration.
    pub admin: Address,
    /// Treasury address that receives collected fees.
    pub treasury: Address,
    /// Fee in basis points charged on escrow release.
    pub fee_bps: i128,
}
