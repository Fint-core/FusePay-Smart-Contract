# Treasury Contract

Platform revenue collection and authorised disbursement.

## Architecture

```
┌─────────────────────────────────────────────┐
│              TreasuryContract               │
├──────────────────┬──────────────────────────┤
│  contract.rs     │  Orchestration           │
│  storage.rs      │  Config + token ledgers  │
│  validation.rs   │  Amount + threshold      │
│  utils.rs        │  Balance arithmetic      │
│  events.rs       │  Accounting events       │
│  errors.rs       │  Error codes 400–499     │
│  types.rs        │  TokenLedger + Config    │
└──────────────────┴──────────────────────────┘
```

## Storage Layout

| Key | Type | Description |
|-----|------|-------------|
| `Initialized` | `bool` | Init flag |
| `Config` | `TreasuryConfig` | Admin + multisig + threshold |
| `TokenLedger(token)` | `TokenLedger` | Per-token accounting |

## Public API

| Function | Auth | Description |
|----------|------|-------------|
| `initialize(admin, multisig, threshold)` | admin | One-time init |
| `deposit(sender, token, amount)` | sender | Accept fee deposit |
| `withdraw(admin, token, amount, recipient)` | admin | Single-signer withdrawal |
| `distribute(admin, token, entries)` | admin | Batch distribution |
| `update_multisig_threshold(admin, threshold)` | admin | Update threshold |
| `get_token_ledger(token)` | — | View accounting totals |
| `get_config()` | — | View config |

## Multisig Threshold

Withdrawals **at or above** `multisig_threshold` return `RequiresMultisig`.
The calling flow must first obtain approval from the multisig contract.

```
amount < threshold  → admin single-signer ✓
amount >= threshold → requires multisig approval
```

## Accounting Model

```
available = total_deposited - total_distributed
```

`total_deposited` and `total_distributed` are monotonically increasing
counters — they are never decremented, providing an auditable history.

## Events

| Event | Trigger |
|-------|---------|
| `fp_treasury::deposit` | `deposit` |
| `fp_treasury::withdraw` | `withdraw` |
| `fp_treasury::fee_coll` | fee deposit from escrow |
| `fp_treasury::distrib` | `distribute` |

## Security Notes

- State updated before token transfers (CEI pattern)
- Admin cannot bypass the multisig threshold by splitting transactions
- Treasury does not hold admin-level access to other contracts
