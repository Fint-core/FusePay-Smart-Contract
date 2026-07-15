# Contributing to FusePay Contracts

Thank you for your interest in contributing to FusePay! This document provides guidelines and standards for contributing to this repository.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Requirements](#testing-requirements)
- [Security Considerations](#security-considerations)
- [Pull Request Process](#pull-request-process)
- [Contract Change Policy](#contract-change-policy)

---

## Code of Conduct

All contributors are expected to adhere to the [Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). Be respectful, inclusive, and constructive.

---

## Getting Started

1. Fork the repository and clone your fork:
   ```bash
   git clone https://github.com/<your-username>/fusepay-contracts.git
   cd fusepay-contracts
   ```

2. Set up the development environment:
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install --locked soroban-cli --features opt
   make setup
   ```

3. Create a feature branch from `main`:
   ```bash
   git checkout -b feat/your-feature-name
   ```

---

## Development Workflow

### Branching Strategy

| Branch pattern | Purpose |
|---|---|
| `main` | Stable, audited code only |
| `develop` | Integration branch |
| `feat/<name>` | New features |
| `fix/<name>` | Bug fixes |
| `security/<name>` | Security patches (private until disclosed) |
| `chore/<name>` | Tooling, docs, refactors |

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short description>

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `security`, `docs`, `test`, `refactor`, `chore`, `build`

Examples:
```
feat(escrow): add timeout-based automatic refund
fix(wallet): prevent zero-amount deposits
security(multisig): validate threshold cannot exceed owner count
docs(treasury): add storage diagram to README
```

---

## Coding Standards

### Rust Style

- Run `cargo fmt` before every commit
- Run `cargo clippy -- -D warnings` and resolve all warnings
- All public functions must have doc comments (`///`)
- No `unwrap()`, `expect()`, or `panic!()` in contract code — return typed errors
- No magic numbers — define all constants in `shared-constants` or the contract's `types.rs`
- Prefer `i128` for token amounts (Soroban standard)
- All arithmetic operations must use checked/saturating variants or SDK-safe helpers

### Contract Architecture

Every contract must follow the layered module structure:

```
src/
├── lib.rs          # Crate root — re-exports and #[contract] attribute
├── interface.rs    # Public trait defining the contract API
├── contract.rs     # #[contractimpl] block — thin orchestration only
├── storage.rs      # All ledger reads/writes — no business logic
├── types.rs        # Contract-specific types and enums
├── errors.rs       # ContractError enum
├── events.rs       # Event emission helpers
├── validation.rs   # Input validation functions
└── utils.rs        # Pure helper functions
```

- `contract.rs` must only call `validation.rs`, `storage.rs`, and `events.rs` — no raw ledger access
- Business logic belongs in `utils.rs` or dedicated modules, not in `contract.rs`
- Storage keys must be defined as typed enums in `storage.rs`

### Error Handling

```rust
// Good
#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    Unauthorized       = 1,
    InvalidAmount      = 2,
    InsufficientFunds  = 3,
}

// Bad — never do this
panic!("not authorized");
Err("something went wrong")
```

### Events

Every state-changing function must emit a typed event via `events.rs`.

---

## Testing Requirements

All contributions must include tests. The minimum bar:

| Change type | Required coverage |
|---|---|
| New function | Unit test for happy path + at least 2 error cases |
| Bug fix | Regression test that fails before the fix |
| Security patch | Security test demonstrating the vulnerability is closed |
| New contract | Full unit test suite + integration test |

### Running Tests

```bash
# Unit tests
make test

# Integration tests
make integration-test

# All tests
make test-all
```

Tests must pass with zero warnings before a PR can be merged.

---

## Security Considerations

- **Never commit secrets** — no private keys, mnemonics, or credentials
- **Do not introduce new dependencies** without team review — supply-chain attacks are real
- If you discover a security vulnerability, follow the responsible disclosure process in [`SECURITY.md`](./SECURITY.md) — do NOT open a public issue
- All new privileged operations must go through `access-control`
- Treasury operations above configured thresholds must route through `multisig`

---

## Pull Request Process

1. Ensure `make ci` passes locally before opening a PR
2. Fill out the PR template completely
3. Link to any relevant issues
4. Request review from at least **two** core maintainers
5. All CI checks must be green
6. PRs touching contract logic require a **security review** label before merge
7. PRs that change storage layout must include a migration note

### PR Checklist

- [ ] `cargo fmt` applied
- [ ] `cargo clippy` passes with zero warnings
- [ ] Unit tests added or updated
- [ ] Integration tests updated if cross-contract behaviour changed
- [ ] Events emitted for all state changes
- [ ] Error cases return typed errors
- [ ] Doc comments updated
- [ ] `CHANGELOG.md` updated
- [ ] No new `unwrap()` or `panic!()` in contract paths

---

## Contract Change Policy

Because deployed contracts may hold user funds, changes to contract logic are subject to stricter review:

1. **Additive changes** (new functions, new events) — standard PR review
2. **Logic changes** (modifying existing function behaviour) — requires two security reviewer approvals and a detailed impact analysis
3. **Storage layout changes** — requires a migration plan and backward-compatibility analysis
4. **Breaking interface changes** — requires a versioning strategy and community discussion issue first

Thank you for helping make FusePay more secure and reliable for everyone.
