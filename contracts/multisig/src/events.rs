//! Event emission for the multisig contract.

use crate::types::Vote;
use soroban_sdk::{symbol_short, Address, Env, Symbol};

fn topic(env: &Env) -> Symbol {
    Symbol::new(env, "fp_multisig")
}

/// Emitted when a new proposal is submitted.
pub fn emit_proposal_created(
    env: &Env,
    proposal_id: u64,
    proposer: &Address,
    expires_at_ledger: u32,
) {
    env.events().publish(
        (topic(env), symbol_short!("prop_new"), proposer.clone()),
        (proposal_id, expires_at_ledger, env.ledger().sequence()),
    );
}

/// Emitted when an owner votes on a proposal.
pub fn emit_vote_cast(env: &Env, proposal_id: u64, voter: &Address, vote: &Vote) {
    env.events().publish(
        (topic(env), symbol_short!("voted"), voter.clone()),
        (proposal_id, vote.clone(), env.ledger().sequence()),
    );
}

/// Emitted when a proposal reaches threshold and is executed.
pub fn emit_proposal_executed(env: &Env, proposal_id: u64, executor: &Address) {
    env.events().publish(
        (topic(env), symbol_short!("executed"), executor.clone()),
        (proposal_id, env.ledger().sequence()),
    );
}

/// Emitted when a proposal is rejected (rejection count >= threshold).
pub fn emit_proposal_rejected(env: &Env, proposal_id: u64) {
    env.events().publish(
        (topic(env), symbol_short!("rejected")),
        (proposal_id, env.ledger().sequence()),
    );
}

/// Emitted when a proposal is explicitly expired by a caller.
pub fn emit_proposal_expired(env: &Env, proposal_id: u64, caller: &Address) {
    env.events().publish(
        (topic(env), symbol_short!("expired"), caller.clone()),
        (proposal_id, env.ledger().sequence()),
    );
}
