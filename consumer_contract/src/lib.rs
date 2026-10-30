#![no_std]
use soroban_sdk::{
    contract, contractimpl, contractclient, Address, Bytes, Env, Vec,
    crypto::bn254::Bn254Fr, symbol_short,
};

#[contractclient(name = "VerifierClient")]
pub trait Verifier {
    fn verify(proof: Bytes, public_inputs: Vec<Bn254Fr>) -> bool;
}

#[contract]
pub struct PrivatePayoutContract;

#[contractimpl]
impl PrivatePayoutContract {
    pub fn claim_funds(
        env: Env,
        verifier_address: Address,
        recipient: Address,
        proof: Bytes,
        commitment: Bn254Fr,
        nullifier_hash: Bn254Fr,
    ) {
        recipient.require_auth();

        // 1. Prevent double spending
        if env.storage().persistent().has(&nullifier_hash) {
            panic!("Nullifier already spent!");
        }

        // 2. Call the lean verifier contract via cross-contract call
        let client = VerifierClient::new(&env, &verifier_address);
        let mut pub_inputs = Vec::new(&env);
        pub_inputs.push_back(commitment);
        pub_inputs.push_back(nullifier_hash.clone());

        let is_valid = client.verify(&proof, &pub_inputs);
        if !is_valid {
            panic!("Invalid zero-knowledge proof!");
        }

        // 3. Mark nullifier as spent and issue payout
        env.storage().persistent().set(&nullifier_hash, &true);
        env.events().publish((symbol_short!("payout"),), recipient);
    }
}
