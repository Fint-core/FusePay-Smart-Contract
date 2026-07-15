# FusePay Contracts

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/fusepay/fusepay-contracts/actions/workflows/ci.yml/badge.svg)](https://github.com/fusepay/fusepay-contracts/actions)
[![Security Audit](https://img.shields.io/badge/security-audited-green)](./docs/audits/)

> Production-ready Soroban smart contracts for the FusePay open-source fintech platform.

---

## Overview

FusePay is an open-source fintech platform that enables wallet management, peer-to-peer payments, escrow, and fee distribution on the Stellar network via Soroban smart contracts.

This repository contains **only the trustless blockchain layer**. Off-chain operations such as airtime top-ups, data bundle purchases, and gift card redemptions are handled by the FusePay backend services and are deliberately excluded from the on-chain layer.

### What lives on-chain

| Contract | Responsibility |
|---|---|
| `wallet` | Wallet ownership, deposits, withdrawals, balance tracking |
| `escrow` | Trustless fund locking, release, refund, and dispute handling |
| `multisig` | Multi-signature proposal, approval, and execution |
| `treasury` | Platform revenue collection and authorized disbursements |
| `access-control` | Role-based permission management (Owner, Admin, Operator, Auditor) |
| `fee-manager` | Fee calculation, discount rules, and referral commissions |
| `oracle` | *(Future)* Price feeds and exchange rate verification |

---

## Repository Structure

```
fusepay-contracts/
├── contracts/              # Individual Soroban smart contracts
│   ├── wallet/
│   ├── escrow/
│   ├── multisig/
│   ├── treasury/
│   ├── access-control/
│   ├── fee-manager/
│   └── oracle/
├── packages/               # Shared reusable libraries
│   ├── shared-types/
│   ├── shared-errors/
│   ├── shared-events/
│   ├── shared-utils/
│   ├── shared-crypto/
│   └── shared-constants/
├── scripts/                # Deployment and utility scripts
│   ├── deploy/
│   └── utils/
├── deployments/            # Deployment manifests per network
│   ├── local/
│   ├── testnet/
│   └── mainnet/
├── integration-tests/      # Cross-contract integration tests
├── docs/                   # Architecture docs and diagrams
│   └── audits/
└── examples/               # Usage examples
```

---

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) >= 1.78
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup) >= 21.0
- [stellar-cli](https://github.com/stellar/stellar-cli)
- Docker (for local Stellar network)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Install Soroban CLI
cargo install --locked soroban-cli --features opt
```

---

## Quick Start

```bash
# Clone the repository
git clone https://github.com/fusepay/fusepay-contracts.git
cd fusepay-contracts

# Build all contracts
make build

# Run all unit tests
make test

# Run security checks
make audit

# Deploy to local network
make deploy-local
```

---

## Building

```bash
# Build all contracts
make build

# Build a specific contract
make build-contract CONTRACT=wallet

# Optimise wasm for deployment
make optimise
```

---

## Testing

```bash
# Unit tests (all contracts)
make test

# Integration tests
make integration-test

# Security-focused tests
make security-test

# Full test suite with coverage
make test-all
```

---

## Deployment

```bash
# Local Stellar network (Docker)
make deploy-local

# Testnet
make deploy-testnet NETWORK_PASSPHRASE="..." SECRET_KEY="..."

# Mainnet (requires hardware key confirmation)
make deploy-mainnet
```

See [`scripts/deploy/`](./scripts/deploy/) and [`deployments/`](./deployments/) for detailed deployment manifests.

---

## Security

FusePay contracts are designed with defence-in-depth:

- Role-based access control on every privileged function
- Multi-signature approval for treasury operations above thresholds
- Overflow-safe arithmetic throughout (Soroban SDK i128/u128)
- Input validation on every public entry point
- No `unwrap()` in production paths — all errors are typed
- Event emission for every state-changing operation
- Replay protection on sensitive operations

Please read [`SECURITY.md`](./SECURITY.md) before reporting vulnerabilities.

---

## Contributing

See [`CONTRIBUTING.md`](./CONTRIBUTING.md) for guidelines on how to contribute.

---

## Audits

Audit reports are published in [`docs/audits/`](./docs/audits/).

---

## License

This project is licensed under the MIT License — see [`LICENSE`](./LICENSE) for details.
