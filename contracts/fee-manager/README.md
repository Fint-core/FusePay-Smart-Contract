# Fee Manager Contract

Centralised fee calculation, discount rules, and referral commissions.

## Fee Types

| Fee | Default Rate | Applies to |
|-----|-------------|------------|
| Platform fee | 50 bps (0.5%) | General operations |
| Escrow fee | 100 bps (1.0%) | Escrow releases |
| Withdrawal fee | 25 bps (0.25%) | Wallet withdrawals |
| Referral commission | 50 bps (0.5% of fee) | Referral programme |

All rates are in basis points. Maximum rate: 500 bps (5%).

## Fee Calculation

```
base_fee     = principal × fee_bps / 10_000
discount     = base_fee × discount_bps / 10_000   (if rule exists)
net_fee      = base_fee - discount
commission   = net_fee × referral_commission_bps / 10_000  (if referrer)
net_principal = principal - net_fee
```

## Discount Rules

- Per-address discount expressed as a percentage of the fee
- Optional expiry ledger — silently ignored after expiry
- 100% discount (`10_000 bps`) zeroes the fee completely

## Public API

| Function | Auth | Description |
|----------|------|-------------|
| `initialize(admin, schedule)` | admin | One-time init |
| `update_fee_schedule(admin, schedule)` | admin | Update all rates |
| `set_discount(admin, rule)` | admin | Add discount rule |
| `remove_discount(admin, address)` | admin | Remove discount |
| `calculate_platform_fee(principal, payer, referrer)` | — | Get fee breakdown |
| `calculate_escrow_fee(principal, payer, referrer)` | — | Get escrow fee |
| `calculate_withdrawal_fee(principal, payer, referrer)` | — | Get withdrawal fee |
| `get_fee_schedule()` | — | View schedule |
| `get_discount(address)` | — | View discount |

## Security Notes

- Fee calculation functions are pure (no state mutation, no token transfers)
- Self-referral (`referrer == payer`) is rejected
- Fee rates are bounded by `MAX_PLATFORM_FEE_BPS` (500 bps)
- Referral commission is a sub-component of the net fee, not an additional charge
