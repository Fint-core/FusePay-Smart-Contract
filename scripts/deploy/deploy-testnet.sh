#!/usr/bin/env bash
# =============================================================================
# FusePay — Testnet deployment script
# Usage: ./scripts/deploy/deploy-testnet.sh <SECRET_KEY>
# =============================================================================
set -euo pipefail

SECRET_KEY="${1:-}"
[[ -z "$SECRET_KEY" ]] && { echo "Usage: $0 <SECRET_KEY>"; exit 1; }

NETWORK="testnet"
NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
RPC_URL="https://soroban-testnet.stellar.org"
WASM_DIR="target/optimised"
DEPLOY_DIR="deployments/testnet"
KEY_NAME="fusepay-testnet-deployer"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

command -v soroban >/dev/null 2>&1 || error "soroban-cli not found"

mkdir -p "$DEPLOY_DIR"

# Import key
info "Importing deployer key..."
echo "$SECRET_KEY" | soroban keys add "$KEY_NAME" --stdin 2>/dev/null || \
    soroban keys add "$KEY_NAME" --secret-key "$SECRET_KEY" 2>/dev/null || \
    warn "Key may already exist"

DEPLOYER=$(soroban keys address "$KEY_NAME")
info "Deployer: $DEPLOYER"

# Fund via Friendbot
info "Funding via Friendbot..."
curl -s "https://friendbot.stellar.org?addr=${DEPLOYER}" > /dev/null && \
    info "Funded." || warn "Friendbot funding failed — account may already exist"

deploy_contract() {
    local name="$1"
    local wasm="$WASM_DIR/fusepay_${name//-/_}.optimised.wasm"

    [[ ! -f "$wasm" ]] && { warn "WASM not found: $wasm"; return; }

    info "Deploying $name to testnet..."
    CONTRACT_ID=$(soroban contract deploy \
        --wasm "$wasm" \
        --source "$KEY_NAME" \
        --network "$NETWORK" \
        --rpc-url "$RPC_URL" \
        --network-passphrase "$NETWORK_PASSPHRASE")

    echo "$CONTRACT_ID" > "$DEPLOY_DIR/${name}.contract_id"
    info "  ✓ $name: $CONTRACT_ID"
}

CONTRACTS=(access_control fee_manager treasury multisig wallet escrow oracle)
for contract in "${CONTRACTS[@]}"; do
    deploy_contract "$contract"
done

# Manifest
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

info "Testnet deployment complete. Manifest: $DEPLOY_DIR/manifest.json"
