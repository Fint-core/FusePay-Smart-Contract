#!/usr/bin/env bash
# =============================================================================
# FusePay — Mainnet deployment script
#
# ⚠️  WARNING: This script deploys to MAINNET and consumes real XLM.
# Only run this after a full audit and community review.
#
# Prerequisites:
#   - DEPLOY_MAINNET_CONFIRMED=yes (set by Makefile guard)
#   - A funded Stellar mainnet account
#   - Hardware key / secure key management (DO NOT pass keys as CLI args)
#
# Usage (via Makefile):
#   make deploy-mainnet DEPLOY_MAINNET_CONFIRMED=yes
# =============================================================================
set -euo pipefail

[[ "${DEPLOY_MAINNET_CONFIRMED:-}" != "yes" ]] && {
    echo "Set DEPLOY_MAINNET_CONFIRMED=yes to proceed."
    exit 1
}

NETWORK="mainnet"
NETWORK_PASSPHRASE="Public Global Stellar Network ; September 2015"
RPC_URL="https://mainnet.sorobanrpc.com"
WASM_DIR="target/optimised"
DEPLOY_DIR="deployments/mainnet"
KEY_NAME="${MAINNET_KEY_NAME:-fusepay-mainnet-deployer}"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

command -v soroban >/dev/null 2>&1 || error "soroban-cli not found"

mkdir -p "$DEPLOY_DIR"

DEPLOYER=$(soroban keys address "$KEY_NAME" 2>/dev/null) || \
    error "Key '$KEY_NAME' not found. Add it with: soroban keys add $KEY_NAME"

info "Deployer: $DEPLOYER"
info "Network:  $NETWORK"
info ""
warn "Deploying to MAINNET. Press Ctrl-C within 10 seconds to abort."
sleep 10

deploy_contract() {
    local name="$1"
    local wasm="$WASM_DIR/fusepay_${name//-/_}.optimised.wasm"

    [[ ! -f "$wasm" ]] && { error "WASM not found: $wasm. Run: make optimise"; }

    info "Deploying $name to mainnet..."
    CONTRACT_ID=$(soroban contract deploy \
        --wasm "$wasm" \
        --source "$KEY_NAME" \
        --network "$NETWORK" \
        --rpc-url "$RPC_URL" \
        --network-passphrase "$NETWORK_PASSPHRASE")

    echo "$CONTRACT_ID" > "$DEPLOY_DIR/${name}.contract_id"
    info "  ✓ $name: $CONTRACT_ID"
}

# Deploy in dependency order
CONTRACTS=(access_control fee_manager multisig treasury wallet escrow oracle)
for contract in "${CONTRACTS[@]}"; do
    deploy_contract "$contract"
done

cat > "$DEPLOY_DIR/manifest.json" <<EOF
{
  "network": "$NETWORK",
  "deployed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "deployer": "$DEPLOYER",
  "rpc_url": "$RPC_URL",
  "contracts": {
$(for c in "${CONTRACTS[@]}"; do
    f="$DEPLOY_DIR/${c}.contract_id"
    [[ -f "$f" ]] && echo "    \"${c}\": \"$(cat "$f")\","
done)
    "_placeholder": "remove this line"
  }
}
EOF

info "Mainnet deployment complete."
info "IMPORTANT: Verify all contract IDs and initialise contracts before announcing."
info "Manifest: $DEPLOY_DIR/manifest.json"
