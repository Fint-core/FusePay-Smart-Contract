//! Public interface trait for the wallet contract.

use crate::errors::WalletError;
use crate::types::Wallet;
use soroban_sdk::{Address, Env};

pub trait WalletInterface {
    /// Initialise the wallet contract with a platform admin address.
    fn initialize(env: Env, admin: Address) -> Result<(), WalletError>;

    /// Create a new wallet for `owner` using `token`.
    ///
    /// One wallet per (owner, token) pair is enforced.
    fn create_wallet(env: Env, owner: Address, token: Address) -> Result<(), WalletError>;

    /// Deposit `amount` of `token` into the wallet owned by `owner`.
    ///
    /// The caller must be the wallet owner. Tokens are transferred from the
    /// caller's token account into this contract.
    fn deposit(
        env: Env,
        owner: Address,
        token: Address,
        amount: i128,
    ) -> Result<(), WalletError>;

    /// Withdraw `amount` of `token` from the wallet owned by `owner`.
    ///
    /// The caller must be the wallet owner. Tokens are transferred to `recipient`.
    fn withdraw(
        env: Env,
        owner: Address,
        token: Address,
        amount: i128,
        recipient: Address,
    ) -> Result<(), WalletError>;

    /// Freeze the wallet, preventing all deposits and withdrawals.
    ///
    /// The caller must be the platform admin.
    fn freeze_wallet(env: Env, admin: Address, owner: Address, token: Address)
        -> Result<(), WalletError>;

    /// Unfreeze a previously frozen wallet.
    ///
    /// The caller must be the platform admin.
    fn unfreeze_wallet(
        env: Env,
        admin: Address,
        owner: Address,
        token: Address,
    ) -> Result<(), WalletError>;

    /// Returns the current balance of a wallet.
    fn get_balance(env: Env, owner: Address, token: Address) -> Result<i128, WalletError>;

    /// Returns the full wallet record.
    fn get_wallet(env: Env, owner: Address, token: Address) -> Result<Wallet, WalletError>;
}
