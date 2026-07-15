# Multisig Contract

Multi-signature approval for privileged FusePay operations.

## Architecture

```
┌───────────────────────────────────────────┐
│            MultisigContract               │
├──────────────────┬────────────────────────┤
│  contract.rs     │  Orchestration        │
│  storage.rs      │  Proposals + votes    │
│  validation.rs   │  Config + expiry      │
│  utils.rs        │  Counter helpers      │
│  events.rs       │  Lifecycle events     │
│  errors.rs       │  Error codes 300–399  │
│  types.rs        │  Proposal + Vote      │
└──────────────────┴────────────────────────┘
```

## Proposal Lifecycle

```
submit_proposal()
       │
       ▼
   [Active] ──── approve() × threshold ──────► [Executed]
       │      ├─ reject() × blocking   ──────► [Rejected]
       └────── expire_proposal() (after expiry) ► [Expired]
```

A proposal becomes permanently **Rejected** when the number of remaining
possible approvals falls below the threshold.

## Storage Layout

| Key | Type | Description |
|-----|------|-------------|
| `Initialized` | `bool` | Init flag |
| `Config` | `MultisigConfig` | Owners + threshold |
| `NextProposalId` | `u64` | Counter |
| `Proposal(id)` | `Proposal` | Proposal record |
| `Vote(id, owner)` | `Vote` | Per-owner vote |

## Public API

| Function | Auth | Description |
|----------|------|-------------|
| `initialize(owners, threshold)` | all owners | One-time init |
| `submit_proposal(proposer, ...)` | owner | Create proposal |
| `approve(owner, id)` | owner | Cast approval |
| `reject(owner, id)` | owner | Cast rejection |
| `expire_proposal(caller, id)` | anyone | Mark expired |
| `is_approved(id)` | — | Check approval status |
| `get_config()` | — | View config |
| `get_proposal(id)` | — | View proposal |

## Integration Pattern

Other contracts (e.g. treasury) call `is_approved(proposal_id)` before
executing privileged actions. The proposal's `call_data` field carries
operation parameters as opaque bytes.

## Security Notes

- Each owner may vote exactly once per proposal
- Votes cannot be changed after submission
- A proposal that has already executed or been rejected cannot receive more votes
- The owner list and threshold are immutable after init — redeployment required for governance changes
