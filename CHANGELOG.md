# Changelog

All notable changes to FusePay Contracts are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Initial workspace setup
- `wallet` contract: ownership, deposits, withdrawals, balance tracking
- `escrow` contract: fund locking, release, refund, dispute, and timeout handling
- `multisig` contract: proposal, approval, rejection, and expiry
- `treasury` contract: revenue collection, fee distribution, authorised withdrawals
- `access-control` contract: Owner, Admin, Operator, Treasury, and Auditor roles
- `fee-manager` contract: platform fee, escrow fee, withdrawal fee, discount rules, referral commission
- `oracle` contract: scaffolding and interface (implementation pending)
- Shared packages: `shared-types`, `shared-errors`, `shared-events`, `shared-utils`, `shared-crypto`, `shared-constants`
- Deployment scripts for local, testnet, and mainnet
- Integration test suite
- GitHub Actions CI pipeline
- Full documentation for all contracts

---

## [0.1.0] - TBD

Initial public release.

[Unreleased]: https://github.com/fusepay/fusepay-contracts/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/fusepay/fusepay-contracts/releases/tag/v0.1.0
