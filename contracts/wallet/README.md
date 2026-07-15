# Wallet Contract

Manages non-custodial wallets on the FusePay platform.

## Architecture

```
┌─────────────────────────────────────────┐
│              WalletContract             │
├─────────────────┬───────────────────────┤
│  contract.rs    │  Thin orchestration   │
│  storage.rs     │  Ledger R/W           │
│  validation.rs  │  Input guards         │
│  utils.rs       │  Safe arithmetic      │
│  events.rs      │  Event emission       │
│  errors.rs      │  Typed errors         │
│  types.rs       │  Wallet struct        │
│  interface.rs   │  Public trait         │
└─────────────────┴───────────────────────┘
```

## Storage Layout

| Key | Type | Description |
|-----|------|-------------|
| `Initialized` | `bool` | Init flag (instance) |
| `Admin` | `Address` | Platform admin (instance) |
| `Wallet(owner, token)` | `Wallet` | Per-wallet record (persistent) |

## Public API

| Function | Auth | Description |
|----------|------|-------------|
| `initialize(admin)` | admin | One-time init |
| `create_wallet(owner, token)` | owner | Create wallet |
| `deposit(owner, token, amount)` | owner | Deposit tokens |
| `withdraw(owner, token, amount, recipient)` | owner | Withdraw tokens |
| `freeze_wallet(admin, owner, token)` | admin | Freeze wallet |
| `unfreeze_wallet(admin, owner, token)` | admin | Unfreeze wallet |
| `get_balance(owner, token)` | — | Query balance |
| `get_wallet(owner, token)` | — | Full wallet record |

## Events

| Event | Trigger |
|-------|---------|
| `fp_wallet::created` | `create_wallet` |
| `fp_wallet::deposit` | `deposit` |
| `fp_wallet::withdraw` | `withdraw` |
| `fp_wallet::frozen` | `freeze_wallet` |
| `fp_wallet::unfrozn` | `unfreeze_wallet` |

## Security Notes

- State is updated **before** token transfers (CEI pattern)
- Admin can freeze wallets but cannot withdraw funds
- Negative and zero amounts are rejected
- Each (owner, token) pair has exactly one wallet

## Example Usage

```rust
// Create a wallet
wallet_client.create_wallet(&owner, &usdc_token);

// Deposit 10 XLM
wallet_client.deposit(&owner, &xlm_token, &100_000_000);

// Withdraw 5 XLM to another address
wallet_client.withdraw(&owner, &xlm_token, &50_000_000, &recipient);
```
