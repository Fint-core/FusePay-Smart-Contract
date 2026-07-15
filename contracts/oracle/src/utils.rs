//! Utility functions for the oracle contract.

use crate::errors::OracleError;

/// Convert a price between two feeds via a common quote asset.
///
/// `price_a` = base_a / quote (e.g. XLM/USDC)
/// `price_b` = base_b / quote (e.g. BTC/USDC)
///
/// Returns base_a / base_b = price_a / price_b, scaled by `decimals`.
///
/// Both feeds must use the same decimals for this to be accurate.
pub fn cross_rate(
    price_a: i128,
    price_b: i128,
    scale: i128,
) -> Result<i128, OracleError> {
    if price_b == 0 {
        return Err(OracleError::ArithmeticOverflow);
    }
    price_a
        .checked_mul(scale)
        .and_then(|p| p.checked_div(price_b))
        .ok_or(OracleError::ArithmeticOverflow)
}
