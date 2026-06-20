# FusePay Smart Contracts

This repository contains the smart contracts used by the FusePay platform, built on the Stellar network using the Soroban framework.

These contracts enable secure, transparent, and decentralized escrow transactions for the FusePay peer-to-peer utility marketplace.

---

## Overview

Smart contracts in FusePay are used to:

* Lock funds in escrow during marketplace trades
* Securely manage transactions between buyers and sellers
* Ensure transparent and tamper-proof trade execution
* Provide on-chain verification of marketplace activity

These contracts significantly improve trust and reliability within the FusePay ecosystem.

---

## Tech Stack

* Rust
* Soroban
* Stellar SDK
* Soroban CLI

---

## Folder Structure

contracts/
├── escrow/             # Escrow contract for marketplace trades
│   └── lib.rs
├── wallet/             # Smart wallet contract
│   └── lib.rs
└── trade/              # Trade execution logic
└── lib.rs

scripts/
├── deploy.sh           # Deployment scripts using Soroban CLI

test/
├── escrow_test.rs      # Escrow contract tests
├── wallet_test.rs      # Wallet contract tests

Cargo.toml              # Rust project configuration
soroban.toml            # Soroban configuration

---

## Smart Contract Responsibilities

* Escrow fund management
* Secure trade execution
* On-chain transaction verification
* Event logging for marketplace activities

---

## Development

### Install dependencies

Make sure you have Rust and Soroban CLI installed.

```bash
rustup update
cargo install --locked soroban-cli
```

---

### Build contracts

```bash
cargo build --target wasm32-unknown-unknown --release
```

---

### Run tests

```bash
cargo test
```

---

### Deploy contracts

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/contract.wasm \
  --source <your-key> \
  --network testnet
```

---

## Security

All smart contracts must be thoroughly tested and reviewed before deployment.

Key practices include:

* Unit and integration testing
* Code reviews
* Secure handling of escrow logic
* Validation of all inputs and state transitions

---

## Contribution

Developers can contribute by:

* Improving contract logic and performance
* Adding new features to escrow and trade systems
* Enhancing test coverage
* Reviewing and auditing smart contracts

Please follow the contribution guidelines in the main repository.
