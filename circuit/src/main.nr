// Noir circuit: Prover proves knowledge of a secret 'nullifier' and 'secret_preimage'
// that hashes to a public commitment, without revealing either.
use dep::std::hash::pedersen_hash;

fn main(
    secret_preimage: Field,      // Private
    nullifier: Field,            // Private
    public_commitment: pub Field,// Public Input 1
    nullifier_hash: pub Field    // Public Input 2
) {
    // 1. Verify membership commitment
    let computed_commitment = pedersen_hash([secret_preimage, nullifier]);
    assert(computed_commitment == public_commitment);

    // 2. Verify nullifier hash to prevent double-spending
    let computed_nullifier = pedersen_hash([nullifier, 0]);
    assert(computed_nullifier == nullifier_hash);
}
