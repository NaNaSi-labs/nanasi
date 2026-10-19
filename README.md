# Nanasi (ナナシ)

**Ultra-Lean Zero-Knowledge Verifier Precompiler for Stellar Soroban**

`nanasi` is a developer tool (analogous to Barretenberg's `bb write_solidity_verifier` or Starknet's `garaga`) that ingests public verification keys from **Circom / SnarkJS** and **Noir (Aztec / Barretenberg)** and outputs production-ready, ultra-lean **Soroban smart contracts**.

---

## The Problem Solved

1. **The 64 KB WASM Limit:**
   Compiling standard cryptographic frameworks (like `arkworks` or `bellman`) to WebAssembly produces binaries between **200 KB and 500 KB**, which Soroban rejects at deployment.
2. **The $O(N)$ Public Input Loop:**
   Traditional verifier implementations compute public input linear combinations via iterative `g1_mul` and `g1_add` loops, causing $2N$ host-boundary crossings.
3. **No Native Code Generators on Stellar:**
   Prior to `nanasi`, developers had to hand-craft custom `build.rs` scripts and manually assemble verification keys.

---

## How Nanasi Works

`nanasi` eliminates all generic big-integer and polynomial algebra crates from the smart contract by:
- **Embedding Verification Keys at Compile-Time:** Zero ledger storage read fees.
- **$O(1)$ Batched Multi-Scalar Multiplication (`g1_msm`):** Passing vectors directly to Stellar's native Pippenger algorithm in a single host call.
- **Native Host Pairing Delegation (`CAP-0074`):** Delegating final pairing checks directly to native validator assembly.

**Result:**
- **WASM Binary Size:** `< 15 KB` (Circom) / `< 20 KB` (Noir) — well below Soroban's 64 KB cap.
- **Execution Cost:** Reduced from `> 120M` WASM instructions to **~3M – 8M CPU instructions**.

---

## CLI Usage

### 1. Circom / SnarkJS Groth16
Export your verification key with `snarkjs`:
```bash
snarkjs zkey export verification_key circuit_final.zkey verification_key.json
```
Generate your lean Soroban contract:
```bash
nanasi circom --vk verification_key.json --out ./contracts/circom_verifier
```

### 2. Noir / UltraHonk (Aztec)
Export your verification key with `bb`:
```bash
bb write_vk -b ./target/circuit.json -o ./target/vk.json
```
Generate your lean Soroban contract:
```bash
nanasi noir --vk ./target/vk.json --out ./contracts/noir_verifier
```

---

## Verification Architectures

### Groth16 (Circom)
Verifies the standard 4-pairing relation natively on the validator:
$$e(-A, B) \cdot e(\alpha, \beta) \cdot e(vk_x, \gamma) \cdot e(C, \delta) == 1$$
Where $vk_x = \text{IC}_0 + \text{MSM}(\text{IC}_{1..n}, \text{public\_inputs})$.

### UltraHonk (Noir)
Folds wire commitments, permutation accumulators, and precomputed selectors using single-call native `g1_msm`, followed by a native 2-pairing KZG opening check:
$$e(P_1, [x]_2) \cdot e(P_2, [1]_2) == 1$$
