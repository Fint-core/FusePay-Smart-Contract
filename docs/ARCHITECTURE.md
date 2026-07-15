# FusePay Contracts — Architecture

## Overview

FusePay's blockchain layer is a set of modular Soroban smart contracts deployed
on the Stellar network. Each contract has a single, well-defined responsibility.
Contracts communicate via cross-contract calls using the Soroban SDK client
pattern.

## Contract Dependency Graph

```
                     ┌──────────────────┐
                     │  access-control  │  ← role registry
                     └────────┬─────────┘
                              │ (cross-contract role checks)
          ┌───────────────────┼───────────────────┐
          │                   │                   │
   ┌──────▼──────┐    ┌───────▼──────┐   ┌───────▼──────┐
   │   wallet    │    │    escrow    │   │  fee-manager │
   └─────────────┘    └──────┬───────┘   └──────────────┘
                             │ (fee transfer)
                      ┌──────▼──────┐
                      │  treasury   │ ← revenue collection
                      └──────┬──────┘
                             │ (large withdrawal approval)
                      ┌──────▼──────┐
                      │  multisig   │
                      └─────────────┘

   ┌──────────────┐
   │    oracle    │  ← future module (price feeds)
   └──────────────┘
```

## Design Principles

### 1. Separation of concerns
Each contract does exactly one thing. The wallet does not calculate fees;
fee-manager does not hold tokens; treasury does not manage roles.

### 2. Trustless operations only
Off-chain operations (airtime, data bundles, gift cards) are handled by the
FusePay backend. Only operations that require trustlessness (escrow, fee
custody, treasury governance) live on-chain.

### 3. Checks-Effects-Interactions (CEI)
All state is updated before any external token transfer. This eliminates
reentrancy as an attack surface in the Soroban model.

### 4. Typed errors — no panics
Every function returns `Result<T, ContractError>`. No `unwrap()`, `panic!()`,
or `expect()` appear in production contract paths.

### 5. Event-driven auditability
Every state-changing operation emits a structured event. Off-chain indexers
can reconstruct the full platform history from events alone.

### 6. Storage isolation
Each contract manages its own storage namespace. No contract writes to
another contract's storage.

## Data Flow: Escrow Release

```
Buyer ──► escrow::release_escrow()
               │
               ├──► token::transfer(contract → beneficiary, net_amount)
               └──► token::transfer(contract → treasury, fee_amount)
                         │
                         treasury::TokenLedger.available += fee_amount
```

## Data Flow: Large Treasury Withdrawal

```
Owner ──► multisig::submit_proposal()
                │
Owner 2 ──► multisig::approve()    ─ threshold reached ─►
                                                          │
Owner 3 ──► multisig::approve()                          │
                                                          ▼
Admin ──► treasury::withdraw()  ← checks is_approved(proposal_id)
               │
               └──► token::transfer(treasury → recipient)
```

## Storage TTL Strategy

| Storage type | TTL (bump) | Used for |
|---|---|---|
| Instance | 30 days | Config, flags, counters |
| Persistent | 1 year | User data, escrows, wallets |
| Temporary | 1 day | Transient computation (unused currently) |

TTLs are extended on every read/write to keep active data alive.

## Upgrade Path

Soroban contracts can be upgraded by uploading a new wasm hash and calling
`env.deployer().update_current_contract_wasm(new_hash)`. This operation
must be governed by the multisig contract for production deployments.
