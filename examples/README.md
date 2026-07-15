# Examples

Usage examples for FusePay contracts.

## Wallet

```rust
// Initialize
wallet_client.initialize(&admin_address);

// Create a wallet for a user
wallet_client.create_wallet(&user_address, &usdc_contract);

// Deposit 10 USDC (assuming 7 decimal places = 100_000_000 units)
wallet_client.deposit(&user_address, &usdc_contract, &100_000_000);

// Withdraw 5 USDC to another address
wallet_client.withdraw(&user_address, &usdc_contract, &50_000_000, &recipient);

// Query balance
let balance = wallet_client.get_balance(&user_address, &usdc_contract);
```

## Escrow

```rust
// Create an escrow for a marketplace order
let params = CreateEscrowParams {
    depositor: buyer,
    beneficiary: seller,
    arbiter: platform_arbiter,
    token: usdc_contract,
    amount: 1_000_000_000, // 100 USDC
    timeout_ledgers: 0,    // use default (24h)
    metadata: String::from_str(&env, "order-12345"),
};
let escrow_id = escrow_client.create_escrow(&params);

// Buyer funds the escrow
escrow_client.fund_escrow(&buyer, &escrow_id);

// After delivery, buyer releases
escrow_client.release_escrow(&buyer, &escrow_id);

// Or raise a dispute
escrow_client.raise_dispute(&buyer, &escrow_id);
// Arbiter resolves
escrow_client.resolve_dispute(&arbiter, &escrow_id, &true /* release */);
```

## Fee Calculation

```rust
// Get fee breakdown before executing a withdrawal
let breakdown = fee_manager_client.calculate_withdrawal_fee(
    &withdrawal_amount,
    &user_address,
    &Some(referrer_address), // optional referrer
);

println!("Base fee: {}", breakdown.base_fee);
println!("Net fee:  {}", breakdown.net_fee);
println!("User receives: {}", breakdown.net_principal);
println!("Referral commission: {}", breakdown.referral_commission);
```

## Multisig

```rust
// Submit a treasury withdrawal proposal
let proposal_id = multisig_client.submit_proposal(
    &proposer,
    &String::from_str(&env, "Q4 revenue distribution"),
    &treasury_contract_address,
    &Bytes::new(&env), // call_data
    &0,               // use default expiry
);

// Owners approve
multisig_client.approve(&owner_1, &proposal_id);
multisig_client.approve(&owner_2, &proposal_id);

// Check approval (used by treasury before executing)
assert!(multisig_client.is_approved(&proposal_id));
```
