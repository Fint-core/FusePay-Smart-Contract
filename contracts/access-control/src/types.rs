//! Types specific to the access-control contract.

use soroban_sdk::{contracttype, Address, Vec};

pub use shared_types::Role;

/// The complete role assignment record stored for a given address.
#[contracttype]
#[derive(Clone, Debug)]
pub struct RoleAssignment {
    /// The address that holds the role.
    pub holder: Address,
    /// The role granted to this address.
    pub role: Role,
    /// Ledger sequence number when the role was granted.
    pub granted_at_ledger: u32,
    /// Address of the admin who granted the role.
    pub granted_by: Address,
}

/// Snapshot of the full access-control configuration.
/// Returned by the `get_config` view function.
#[contracttype]
#[derive(Clone, Debug)]
pub struct AccessConfig {
    /// Current contract owner.
    pub owner: Address,
    /// All current role assignments (sorted by role then address).
    pub assignments: Vec<RoleAssignment>,
}
