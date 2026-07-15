//! # shared-crypto
//!
//! Cryptographic helpers shared across FusePay contracts.
//!
//! Soroban exposes SHA-256 and Keccak-256 via the host environment.
//! This crate wraps those host functions with typed helpers to avoid
//! raw byte manipulation scattered across contracts.

#![no_std]

use soroban_sdk::{Bytes, BytesN, Env};

// ── Hashing ────────────────────────────────────────────────────────────────

/// Compute the SHA-256 hash of the given bytes using the Soroban host.
///
/// Returns a 32-byte fixed-length array.
pub fn sha256(env: &Env, data: &Bytes) -> BytesN<32> {
    env.crypto().sha256(data).into()
}

/// Compute the Keccak-256 hash of the given bytes using the Soroban host.
///
/// Returns a 32-byte fixed-length array.
pub fn keccak256(env: &Env, data: &Bytes) -> BytesN<32> {
    env.crypto().keccak256(data).into()
}

// ── Commitment scheme ──────────────────────────────────────────────────────

/// Build a canonical pre-image for a commitment hash.
///
/// Concatenates `[prefix | payload]` into a single `Bytes` value that can
/// then be passed to `sha256` or `keccak256`.
///
/// Used by the escrow and multisig contracts to bind operations to a
/// specific sender and sequence number, providing basic replay protection.
pub fn build_commitment_preimage(env: &Env, prefix: &[u8], payload: &Bytes) -> Bytes {
    let mut preimage = Bytes::new(env);
    preimage.extend_from_slice(prefix);
    preimage.append(payload);
    preimage
}

/// Compute a commitment hash: SHA-256(prefix || payload).
pub fn commitment_hash(env: &Env, prefix: &[u8], payload: &Bytes) -> BytesN<32> {
    let preimage = build_commitment_preimage(env, prefix, payload);
    sha256(env, &preimage)
}

// ── Ed25519 signature verification ────────────────────────────────────────

/// Verify an Ed25519 signature using the Soroban host.
///
/// # Arguments
/// * `public_key` – 32-byte Ed25519 public key.
/// * `message`    – The message that was signed.
/// * `signature`  – 64-byte Ed25519 signature.
///
/// The Soroban host will trap if the signature is invalid.
pub fn verify_ed25519(
    env: &Env,
    public_key: &BytesN<32>,
    message: &Bytes,
    signature: &BytesN<64>,
) {
    env.crypto()
        .ed25519_verify(public_key, message, signature);
}
