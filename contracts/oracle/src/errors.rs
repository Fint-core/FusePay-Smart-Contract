//! Oracle contract error codes (range 700–799).

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum OracleError {
    AlreadyInitialized    = 700,
    NotInitialized        = 701,
    Unauthorized          = 702,
    /// The requested price feed does not exist.
    FeedNotFound          = 703,
    /// The price data is stale (beyond max_staleness_ledgers).
    StalePriceFeed        = 704,
    /// The submitted price is outside acceptable bounds.
    PriceOutOfBounds      = 705,
    /// The feed submitter is not an authorised price provider.
    UnauthorizedProvider  = 706,
    /// Arithmetic overflow in price calculation.
    ArithmeticOverflow    = 707,
    /// A feed for this asset pair already exists.
    FeedAlreadyExists     = 708,
    /// The price value is zero or negative.
    InvalidPrice          = 709,
}
