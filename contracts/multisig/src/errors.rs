//! Multisig contract error codes (range 300–399).

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum MultisigError {
    AlreadyInitialized = 300,
    NotInitialized = 301,
    Unauthorized = 302,
    ProposalNotFound = 303,
    InvalidState = 304,
    /// Threshold must be >= 1 and <= owner count.
    InvalidThreshold = 305,
    /// Owner list is empty.
    NoOwners = 306,
    /// Owner count exceeds the maximum allowed.
    TooManyOwners = 307,
    /// An address appears more than once in the owner list.
    DuplicateOwner = 308,
    /// The caller has already voted on this proposal.
    AlreadyVoted = 309,
    /// The proposal has expired.
    ProposalExpired = 310,
    /// Arithmetic overflow.
    ArithmeticOverflow = 311,
    /// Expiry exceeds the maximum allowed duration.
    ExpiryTooLong = 312,
    /// The proposal is not in Executed state; cannot re-execute.
    AlreadyExecuted = 313,
}
