# Security Policy

## Supported Versions

Only the latest release on `main` is actively supported with security patches.

| Version | Supported |
|---------|-----------|
| latest  | ✅ |
| older   | ❌ |

---

## Reporting a Vulnerability

**Please do NOT open a public GitHub issue for security vulnerabilities.**

FusePay takes security seriously. If you discover a vulnerability — especially one that could result in loss of funds — please follow the responsible disclosure process below.

### How to Report

1. **Email**: Send a detailed report to `security@fusepay.io`
2. **Encryption**: Encrypt your report using our PGP key (published at `https://fusepay.io/.well-known/pgp-key.asc`)
3. **Include**:
   - A clear description of the vulnerability
   - Affected contract(s) and function(s)
   - Steps to reproduce
   - Proof-of-concept code if available
   - Your assessment of severity and impact
   - Your suggested fix if you have one

### Response Timeline

| Action | Timeline |
|---|---|
| Acknowledgement | Within 48 hours |
| Initial assessment | Within 5 business days |
| Patch development | Depends on severity |
| Public disclosure | Coordinated with reporter |

### Severity Classification

| Severity | Description | Examples |
|---|---|---|
| Critical | Direct loss of funds | Reentrancy, unauthorised withdrawal |
| High | Privilege escalation or partial fund loss | Role bypass, fee manipulation |
| Medium | Denial of service or logic errors | Stuck escrow, incorrect accounting |
| Low | Minor issues with limited impact | Gas inefficiency, event omission |

---

## Scope

The following are **in scope** for security reports:

- All contracts in `contracts/`
- Shared packages in `packages/`
- Deployment scripts in `scripts/`
- Integration tests that reveal contract vulnerabilities

The following are **out of scope**:

- FusePay off-chain backend services (separate repository)
- Frontend applications
- Third-party dependencies (report to the upstream project)
- Issues already disclosed in `docs/audits/`

---

## Bug Bounty

FusePay operates a bug bounty programme for critical and high-severity vulnerabilities. Details are published at `https://fusepay.io/bug-bounty`.

---

## Security Architecture

### Defence in Depth

FusePay contracts are designed with multiple security layers:

1. **Access Control** — Every privileged function checks roles via the `access-control` contract before execution
2. **Multi-Signature** — Treasury withdrawals and admin operations above configured thresholds require multi-sig approval
3. **Input Validation** — All public entry points validate inputs before touching state
4. **Safe Arithmetic** — All arithmetic uses checked operations or i128/u128 to prevent overflow
5. **Typed Errors** — All error paths return typed `ContractError` variants — no panics in production
6. **Event Emission** — Every state change emits a structured event for off-chain auditability
7. **Storage Isolation** — Each contract manages its own storage namespace; no cross-contract storage writes

### Known Limitations and Accepted Risks

- Soroban contracts operate in a deterministic sandbox; time-based operations rely on ledger sequence numbers and timestamps which can be influenced by validators within bounds
- Oracle price feeds (future module) introduce external data trust assumptions; feeds must be verified before use in financial calculations
- Multi-sig expiry relies on ledger close time; network congestion can affect timing

### Audit History

| Date | Auditor | Scope | Report |
|---|---|---|---|
| TBD | TBD | All contracts v1.0 | Pending |

Audit reports are published in [`docs/audits/`](./docs/audits/).
