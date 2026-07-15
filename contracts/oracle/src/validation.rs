//! Input validation for the oracle contract.

use crate::errors::OracleError;

/// Validate that a price value is strictly positive.
pub fn validate_price(price: i128) -> Result<(), OracleError> {
    if price <= 0 {
        return Err(OracleError::InvalidPrice);
    }
    Ok(())
}

/// Validate that a staleness window is sensible (> 0 ledgers).
pub fn validate_staleness(max_staleness_ledgers: u32) -> Result<(), OracleError> {
    if max_staleness_ledgers == 0 {
        return Err(OracleError::NotInitialized); // reuse closest error
    }
    Ok(())
}
