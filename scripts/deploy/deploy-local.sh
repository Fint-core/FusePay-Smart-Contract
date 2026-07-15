#!/usr/bin/env bash
# =============================================================================
# FusePay — Local deployment script
# Deploys all contracts to a local Stellar network (Docker).
# =============================================================================
set -euo pipefail

NETWORK="local"
NETWORK_PASSPHRASE="Standalone Network ; February 2017"
RPC_URL="http://localhost:8000/soroban/rpc"
WASM_DIR="target/optimised"
DEPLOY_DIR="deployments/local"
KEY_NAME="fusepay-local"

# ── Colours ───────────────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
info()    { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()    { echo -e "${YELLOW}[WARN]${NC} $*"; }
error()   { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

# ── Preflight checks ──────────────────────────────────────────────────────────
command -v soroban >/dev/null 2>&1 || error "soroban-cli not found. Run: make setup"
command -v stellar >/dev/null 2>&1 || warn "stellar-cli not found — some steps may be skipped"

mkdir -p "$DEPLOY_DIR"

# ── Generate or load deployer key ─────────────────────────────────────────────
if ! soroban keys show "$KEY_NAME" &>/dev/null; then
    info "Generating local deployer key: $KEY_NAME"
    soroban keys generate --no-fund "$KEY_NAME"
fi

DEPLOYER=$(soroban keys address "$KEY_NAME")
info "Deployer: $DEPLOYER"

# Fund the deployer account on local network
info "Funding deployer account..."
soroban keys fund --network "$NETWORK" "$KEY_NAME" 2>/dev/null || warn "Could not fund — continuing"

# ── Deploy function ────────────────────────────────────────────────────────────
deploy_contract() {
    local name="$1"
    local wasm="$WASM_DIR/fusepay_${name//-/_}.optimised.wasm"

    if [[ ! -f "$wasm" ]]; then
        warn "WASM not found for $name: $wasm — skipping"
        return
    fi

    info "Uploading $name..."
    CONTRACT_ID=$(soroban contract deploy \
        --wasm "$wasm" \
        --source "$KEY_NAME" \
        --network "$NETWORK" \
        --rpc-url "$RPC_URL" \
        --network-passphrase "$NETWORK_PASSPHRASE")

    echo "$CONTRACT_ID" > "$DEPLOY_DIR/${name}.contract_id"
    info "  ✓ $name deployed: $CONTRACT_ID"
}

# ── Deploy all contracts ──────────────────────────────────────────────────────
CONTRACTS=(access_control fee_manager treasury multisig wallet escrow oracle)

for contract in "${CONTRACTS[@]}"; do
    deploy_contract "$contract"
done

# ── Write deployment manifest ──────────────────────────────────────────────────
info "Writing deployment manifest..."
cat > "$DEPLOY_DIR/manifest.json" <<EOF
{
  "network": "$NETWORK",
  "deployed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "deployer": "$DEPLOYER",
  "contracts": {
$(for contract in "${CONTRACTS[@]}"; do
    id_file="$DEPLOY_DIR/${contract}.contract_id"
    if [[ -f "$id_file" ]]; then
        id=$(cat "$id_file")
        echo "    \"${contract}\": \"${id}\","
    fi
done)
    "_placeholder": "remove this line"
  }
}
EOF

info "Local deployment complete. Manifest: $DEPLOY_DIR/manifest.json"
