//! Unit tests for the oracle contract.

#![cfg(test)]

use fusepay_oracle::{OracleContract, OracleContractClient, OracleError};
use soroban_sdk::{testutils::{Address as _, Ledger as _}, Address, Env, String};

fn setup(env: &Env) -> (OracleContractClient, Address) {
    let contract_id = env.register_contract(None, OracleContract);
    let client = OracleContractClient::new(env, &contract_id);
    let admin = Address::generate(env);
    client.initialize(&admin, &100); // 100 ledger staleness window
    (client, admin)
}

#[test]
fn test_register_and_submit_price() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    let provider = Address::generate(&env);

    client.add_provider(&admin, &provider);
    client.register_feed(
        &admin,
        &String::from_str(&env, "XLM/USDC"),
        &String::from_str(&env, "XLM"),
        &String::from_str(&env, "USDC"),
        &6,
    );

    client.submit_price(&provider, &String::from_str(&env, "XLM/USDC"), &1_250_000);

    let feed = client.get_price(&String::from_str(&env, "XLM/USDC"));
    assert_eq!(feed.price, 1_250_000);
}

#[test]
fn test_stale_price_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env); // staleness = 100 ledgers
    let provider = Address::generate(&env);

    client.add_provider(&admin, &provider);
    client.register_feed(
        &admin,
        &String::from_str(&env, "XLM/USDC"),
        &String::from_str(&env, "XLM"),
        &String::from_str(&env, "USDC"),
        &6,
    );

    client.submit_price(&provider, &String::from_str(&env, "XLM/USDC"), &1_250_000);

    // Advance past staleness window
    env.ledger().with_mut(|l| {
        l.sequence_number += 200;
    });

    let result = client.try_get_price(&String::from_str(&env, "XLM/USDC"));
    assert!(matches!(result, Err(Ok(OracleError::StalePriceFeed))));
}

#[test]
fn test_unauthorized_provider_cannot_submit() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    let non_provider = Address::generate(&env);

    client.register_feed(
        &admin,
        &String::from_str(&env, "XLM/USDC"),
        &String::from_str(&env, "XLM"),
        &String::from_str(&env, "USDC"),
        &6,
    );

    let result = client.try_submit_price(
        &non_provider,
        &String::from_str(&env, "XLM/USDC"),
        &1_250_000,
    );
    assert_eq!(result, Err(Ok(OracleError::UnauthorizedProvider)));
}

#[test]
fn test_duplicate_feed_registration_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);

    client.register_feed(
        &admin,
        &String::from_str(&env, "XLM/USDC"),
        &String::from_str(&env, "XLM"),
        &String::from_str(&env, "USDC"),
        &6,
    );

    let result = client.try_register_feed(
        &admin,
        &String::from_str(&env, "XLM/USDC"),
        &String::from_str(&env, "XLM"),
        &String::from_str(&env, "USDC"),
        &6,
    );
    assert_eq!(result, Err(Ok(OracleError::FeedAlreadyExists)));
}

#[test]
fn test_zero_price_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    let provider = Address::generate(&env);

    client.add_provider(&admin, &provider);
    client.register_feed(
        &admin,
        &String::from_str(&env, "XLM/USDC"),
        &String::from_str(&env, "XLM"),
        &String::from_str(&env, "USDC"),
        &6,
    );

    let result = client.try_submit_price(&provider, &String::from_str(&env, "XLM/USDC"), &0);
    assert_eq!(result, Err(Ok(OracleError::InvalidPrice)));
}
