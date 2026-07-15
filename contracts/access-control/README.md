# Access Control Contract

Role-based access control (RBAC) for the FusePay platform.

## Role Hierarchy

```
Owner
  └── Admin
        └── Operator / Treasury
              └── Auditor
```

A higher-role holder satisfies checks for all lower roles.

## Roles

| Role | Description |
|------|-------------|
| `Owner` | Superuser — transfers ownership, manages admins |
| `Admin` | Configures parameters, manages operators |
| `Operator` | Day-to-day operations |
| `Treasury` | Treasury withdrawal authorisation |
| `Auditor` | Read-only access |

## Public API

| Function | Auth | Description |
|----------|------|-------------|
| `initialize(owner)` | owner | One-time init |
| `grant_role(caller, grantee, role)` | Owner / Admin | Grant role |
| `revoke_role(caller, holder, role)` | Owner / Admin | Revoke role |
| `transfer_ownership(caller, new_owner)` | Owner | Transfer ownership |
| `has_role(address, role)` | — | Check role |
| `get_role_assignment(address, role)` | — | Full assignment record |
| `get_role_holders(role)` | — | All holders of a role |
| `get_owner()` | — | Current owner |
| `get_config()` | — | Full snapshot |

## Integration

Other contracts call this contract via cross-contract invocation:

```rust
let ac = AccessControlContractClient::new(&env, &access_control_id);
ac.has_role(&caller, &Role::Operator) // returns bool
```

## Security Notes

- The `Owner` role cannot be granted via `grant_role` — use `transfer_ownership`
- Exactly one Owner exists at all times
- Admins cannot grant roles above their own level
