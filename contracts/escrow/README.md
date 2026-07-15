# Escrow Contract

Trustless escrow for marketplace and peer-to-peer transactions.

## Architecture

```
┌───────────────────────────────────────────────────────────┐
│                      EscrowContract                       │
├──────────────────┬────────────────────────────────────────┤
│  contract.rs     │  Orchestration                        │
│  storage.rs      │  Escrow records + ID counter          │
│  validation.rs   │  State + amount + timeout guards      │
│  utils.rs        │  Fee computation                      │
│  events.rs       │  Full lifecycle events                │
│  errors.rs       │  Typed error codes (200–299)          │
│  types.rs        │  Escrow struct + state enum           │
└──────────────────┴────────────────────────────────────────┘
```

## Lifecycle Flow

```
create_escrow()
      │
      ▼
  [Pending] ──── cancel_escrow() ──────► [Cancelled]
      │
  fund_escrow()
      │
      ▼
  [Funded] ──── release_escrow() ──────► [Released]  ← fee sent to treasury
      │      ├─ refund_escrow()  ──────► [Refunded]
      │      └─ claim_timeout_refund() ► [Expired]   (after expiry ledger)
      │
  raise_dispute()
      │
      ▼
  [Disputed] ─── resolve_dispute(true)  ──► [Released]
             └── resolve_dispute(false) ──► [Refunded]
```

## Storage Layout

| Key | Type | Description |
|-----|------|-------------|
| `Initialized` | `bool` | Init flag |
| `Config` | `EscrowConfig` | Admin + treasury + fee |
| `NextEscrowId` | `u64` | Auto-increment counter |
| `Escrow(id)` | `Escrow` | Full escrow record |

## Public API

| Function | Auth | Description |
|----------|------|-------------|
| `initialize(admin, treasury, fee_bps)` | admin | One-time init |
| `create_escrow(params)` | depositor | Create agreement |
| `fund_escrow(depositor, id)` | depositor | Lock funds |
| `release_escrow(caller, id)` | depositor / arbiter | Release to beneficiary |
| `refund_escrow(caller, id)` | beneficiary / arbiter | Refund depositor |
| `raise_dispute(caller, id)` | depositor / beneficiary | Raise dispute |
| `resolve_dispute(arbiter, id, release)` | arbiter | Arbitrate |
| `cancel_escrow(depositor, id)` | depositor | Cancel before funding |
| `claim_timeout_refund(caller, id)` | depositor | Auto-refund after expiry |
| `get_escrow(id)` | — | View escrow |
| `update_fee(admin, fee_bps)` | admin | Change fee rate |

## Events

| Event | Trigger |
|-------|---------|
| `fp_escrow::created` | `create_escrow` |
| `fp_escrow::funded` | `fund_escrow` |
| `fp_escrow::released` | `release_escrow` / `resolve_dispute(true)` |
| `fp_escrow::refunded` | `refund_escrow` / `resolve_dispute(false)` |
| `fp_escrow::disputed` | `raise_dispute` |
| `fp_escrow::canceld` | `cancel_escrow` |
| `fp_escrow::expired` | `claim_timeout_refund` |

## Fee Calculation

```
fee = amount × fee_bps / 10_000
net = amount - fee
```

The fee is sent to the treasury on release only. Refunds return the full amount.

## Security Notes

- All state changes precede external token transfers (CEI pattern)
- Expired escrows cannot be released, only refunded
- Only the depositor can claim a timeout refund
- Arbiter cannot create or fund escrows — only resolve
