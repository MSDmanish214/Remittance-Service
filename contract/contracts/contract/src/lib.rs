#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct RemittanceContract;

#[contractimpl]
impl RemittanceContract {

    // Send funds (record remittance)
    pub fn send_remittance(
        env: Env,
        sender: Address,
        receiver: Address,
        amount: i128,
    ) {
        // Require sender authorization
        sender.require_auth();

        // Store remittance data
        let key = (Symbol::new(&env, "remit"), &sender, &receiver);
        env.storage().persistent().set(&key, &amount);
    }

    // Claim funds
    pub fn claim_remittance(
        env: Env,
        sender: Address,
        receiver: Address,
    ) -> i128 {
        // Require receiver authorization
        receiver.require_auth();

        let key = (Symbol::new(&env, "remit"), &sender, &receiver);

        let amount: i128 = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(0);

        // Remove after claim
        env.storage().persistent().remove(&key);

        amount
    }

    // Check remittance
    pub fn check_remittance(
        env: Env,
        sender: Address,
        receiver: Address,
    ) -> i128 {
        let key = (Symbol::new(&env, "remit"), &sender, &receiver);

        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(0)
    }
}