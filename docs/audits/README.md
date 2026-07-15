# Audit Reports

Security audit reports for FusePay contracts are published here.

## Audit Status

| Version | Auditor | Date | Scope | Status |
|---------|---------|------|-------|--------|
| v1.0 | TBD | TBD | All contracts | Pending |

## Requesting an Audit

Community members and security researchers are encouraged to review the
contracts independently. Please follow the process in [`SECURITY.md`](../../SECURITY.md)
for reporting findings.

## Self-Assessment Checklist

Before each release, maintainers complete the following:

- [ ] `cargo audit` passes with zero findings
- [ ] `cargo clippy -- -D warnings` passes
- [ ] All tests pass on latest Soroban SDK version
- [ ] Storage layout changes reviewed for backward compatibility
- [ ] New external calls reviewed for reentrancy surface
- [ ] All new privileged operations require `require_auth`
- [ ] All new state changes emit events
- [ ] CHANGELOG updated
