# ============================================================
# FusePay Contracts — Makefile
# ============================================================
# Targets are designed to be composable.
# Run `make help` to see all available targets.
# ============================================================

SHELL := /bin/bash
.PHONY: help setup build build-contract optimise test test-all \
        integration-test security-test fmt lint audit clean \
        deploy-local deploy-testnet deploy-mainnet ci check-deps

# ── Configuration ────────────────────────────────────────────
CONTRACTS := wallet escrow multisig treasury access-control fee-manager oracle
WASM_DIR  := target/wasm32-unknown-unknown/release
OUT_DIR   := target/optimised

SOROBAN  ?= soroban
STELLAR  ?= stellar
CONTRACT ?=

# ── Help ─────────────────────────────────────────────────────
help: ## Show this help message
	@echo "FusePay Contracts — available targets"
	@echo "────────────────────────────────────────────────────"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| sort \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-25s\033[0m %s\n", $$1, $$2}'

# ── Setup ────────────────────────────────────────────────────
setup: ## Install required toolchain and targets
	rustup target add wasm32-unknown-unknown
	cargo install --locked soroban-cli --features opt || true
	@echo "Setup complete."

check-deps: ## Verify required tools are installed
	@command -v cargo    >/dev/null 2>&1 || { echo "cargo not found"; exit 1; }
	@command -v soroban  >/dev/null 2>&1 || { echo "soroban-cli not found — run: make setup"; exit 1; }
	@echo "All dependencies present."

# ── Build ────────────────────────────────────────────────────
build: check-deps ## Build all contracts for wasm32
	cargo build --release --target wasm32-unknown-unknown \
		--exclude integration-tests \
		--workspace
	@echo "Build complete."

build-contract: check-deps ## Build a single contract (CONTRACT=<name>)
ifndef CONTRACT
	$(error CONTRACT is not set. Usage: make build-contract CONTRACT=wallet)
endif
	cargo build --release --target wasm32-unknown-unknown \
		-p fusepay-$(CONTRACT)
	@echo "$(CONTRACT) built."

optimise: build ## Optimise all wasm binaries for deployment
	mkdir -p $(OUT_DIR)
	@for contract in $(CONTRACTS); do \
		wasm_file="$(WASM_DIR)/fusepay_$${contract//-/_}.wasm"; \
		if [ -f "$$wasm_file" ]; then \
			$(SOROBAN) contract optimize \
				--wasm "$$wasm_file" \
				--wasm-out "$(OUT_DIR)/fusepay_$${contract//-/_}.optimised.wasm"; \
			echo "Optimised: $$contract"; \
		fi \
	done
	@echo "Optimisation complete. Output: $(OUT_DIR)/"

# ── Test ─────────────────────────────────────────────────────
test: ## Run all unit tests
	cargo test --workspace \
		--exclude integration-tests \
		-- --nocapture 2>&1

test-all: test integration-test security-test ## Run all tests (unit + integration + security)

integration-test: ## Run cross-contract integration tests
	cargo test --package fusepay-integration-tests -- --nocapture 2>&1

security-test: ## Run security-focused test suite
	cargo test --workspace \
		--exclude integration-tests \
		-- security 2>&1

# ── Code Quality ─────────────────────────────────────────────
fmt: ## Format all Rust code
	cargo fmt --all

fmt-check: ## Check formatting without modifying files
	cargo fmt --all -- --check

lint: ## Run clippy on all crates
	cargo clippy --all-targets --all-features \
		--workspace \
		-- -D warnings

audit: ## Run cargo-audit for known CVEs
	@command -v cargo-audit >/dev/null 2>&1 || cargo install cargo-audit
	cargo audit

# ── CI ───────────────────────────────────────────────────────
ci: fmt-check lint test audit ## Full CI pipeline (runs in GitHub Actions)
	@echo "CI pipeline passed."

# ── Deployment ───────────────────────────────────────────────
deploy-local: optimise ## Deploy all contracts to local network
	@bash scripts/deploy/deploy-local.sh

deploy-testnet: optimise ## Deploy to Stellar testnet
ifndef SECRET_KEY
	$(error SECRET_KEY is required for testnet deployment)
endif
	@bash scripts/deploy/deploy-testnet.sh "$(SECRET_KEY)"

deploy-mainnet: optimise ## Deploy to Stellar mainnet (requires explicit confirmation)
	@echo ""
	@echo "⚠️  WARNING: You are about to deploy to MAINNET."
	@echo "   This action will consume real XLM."
	@echo "   Type 'DEPLOY_MAINNET_CONFIRMED=yes' to proceed."
	@echo ""
ifndef DEPLOY_MAINNET_CONFIRMED
	$(error Set DEPLOY_MAINNET_CONFIRMED=yes to deploy to mainnet)
endif
ifneq ($(DEPLOY_MAINNET_CONFIRMED),yes)
	$(error DEPLOY_MAINNET_CONFIRMED must be 'yes')
endif
	@bash scripts/deploy/deploy-mainnet.sh

# ── Clean ────────────────────────────────────────────────────
clean: ## Remove build artefacts
	cargo clean
	rm -rf $(OUT_DIR)
	@echo "Clean complete."
