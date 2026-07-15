//! Input validation for the multisig contract.

use crate::errors::MultisigError;
use shared_constants::{MAX_MULTISIG_EXPIRY_LEDGERS, MAX_MULTISIG_OWNERS, MIN_MULTISIG_THRESHOLD};
use soroban_sdk::{Address, Env, Vec};

/// Validate the owner list and threshold configuration.
pub fn validate_config(
    owners: &Vec<Address>,
    threshold: u32,
) -> Result<(), MultisigError> {
    if owners.is_empty() {
        return Err(MultisigError::NoOwners);
    }
    let count = owners.len();
    if count > MAX_MULTISIG_OWNERS {
        return Err(MultisigError::TooManyOwners);
    }
    if threshold < MIN_MULTISIG_THRESHOLD || threshold > count {
        return Err(MultisigError::InvalidThreshold);
    }

    // Check for duplicates in O(n^2) — acceptable for small owner sets.
    for i in 0..owners.len() {
        for j in (i + 1)..owners.len() {
            if owners.get(i) == owners.get(j) {
                return Err(MultisigError::DuplicateOwner);
            }
        }
    }

    Ok(())
}

/// Validate expiry duration.
pub fn validate_expiry(timeout_ledgers: u32) -> Result<(), MultisigError> {
    if timeout_ledgers > MAX_MULTISIG_EXPIRY_LEDGERS {
        return Err(MultisigError::ExpiryTooLong);
    }
    Ok(())
}

/// Assert the proposal has not passed its expiry ledger.
pub fn require_not_expired(env: &Env, expires_at_ledger: u32) -> Result<(), MultisigError> {
    if env.ledger().sequence() >= expires_at_ledger {
        return Err(MultisigError::ProposalExpired);
    }
    Ok(())
}

/// Assert the given address is in the owner list.
pub fn require_owner(owners: &Vec<Address>, caller: &Address) -> Result<(), MultisigError> {
    if owners.contains(caller) {
        return Ok(());
    }
    Err(MultisigError::Unauthorized)
}
