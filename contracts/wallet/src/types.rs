//! Types specific to the wallet contract.

use soroban_sdk::{contracttype, Address};

/// Full wallet record stored on-chain.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Wallet {
    /// Stellar address of the wallet owner.
    pub owner: Address,
    /// Token contract address this wallet operates with.
    pub token: Address,
    /// Current balance in the token's smallest unit.
    pub balance: i128,
    /// Whether the wallet is active. A frozen wallet cannot deposit or withdraw.
    pub is_active: bool,
    /// Ledger sequence when the wallet was created.
    pub created_at_ledger: u32,
    /// Ledger sequence of the last transaction (deposit or withdrawal).
    pub last_tx_ledger: u32,
    /// Running total of all deposits (never decremented).
    pub total_deposited: i128,
    /// Running total of all withdrawals (never decremented).
    pub total_withdrawn: i128,
}
