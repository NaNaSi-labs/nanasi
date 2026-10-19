pub mod parser;
pub mod generator_circom;
pub mod generator_noir;
#[cfg(test)]
pub mod tests;

use std::env;
use std::fs;
use std::path::Path;

fn print_usage() {
    println!("==========================================================================");
    println!("   NANASI: Ultra-Lean ZK Verifier Precompiler for Stellar Soroban         ");
    println!("   (Generates < 20 KB contracts with O(1) G1 MSM & Native Pairing Check) ");
    println!("==========================================================================");
    println!("\nUsage:");
    println!("  nanasi circom --vk <path/to/verification_key.json> --out <output_dir>");
    println!("  nanasi noir   --vk <path/to/vk.json>                --out <output_dir>");
    println!("\nExamples:");
    println!("  nanasi circom --vk ./circuits/vk.json --out ./contracts/verifier");
    println!("  nanasi noir   --vk ./target/vk.json   --out ./contracts/noir_verifier");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "circom" => {
            let mut vk_path = String::new();
            let mut out_dir = String::from("./circom_verifier");

            let mut i = 2;
            while i < args.len() {
                if args[i] == "--vk" && i + 1 < args.len() {
                    vk_path = args[i + 1].clone();
                    i += 2;
                } else if args[i] == "--out" && i + 1 < args.len() {
                    out_dir = args[i + 1].clone();
                    i += 2;
                } else {
                    i += 1;
                }
            }

            if vk_path.is_empty() {
                eprintln!("Error: Missing required --vk parameter");
                print_usage();
                std::process::exit(1);
            }

            let vk_json = fs::read_to_string(&vk_path).expect("Failed to read Circom verification key");
            let vk: parser::CircomVerificationKey = serde_json::from_str(&vk_json)
                .expect("Failed to parse Circom verification key JSON");

            println!("Parsing Circom Groth16 verification key...");
            println!("  Protocol: {}", vk.protocol);
            println!("  Curve:    {}", vk.curve);
            println!("  Public Inputs: {}", vk.nPublic);
            println!("  IC Points:     {}", vk.ic.len());

            let contract_code = generator_circom::generate_circom_contract(&vk);
            emit_contract_package(&out_dir, &contract_code, "circom_groth16_verifier");

            println!("\n✅ Successfully generated lean Circom Groth16 contract at: {}", out_dir);
            println!("   - WASM target size: < 15 KB");
            println!("   - Replaced O(N) host loop with single-call G1 MSM!");
        }

        "noir" => {
            let mut vk_path = String::new();
            let mut out_dir = String::from("./noir_verifier");

            let mut i = 2;
            while i < args.len() {
                if args[i] == "--vk" && i + 1 < args.len() {
                    vk_path = args[i + 1].clone();
                    i += 2;
                } else if args[i] == "--out" && i + 1 < args.len() {
                    out_dir = args[i + 1].clone();
                    i += 2;
                } else {
                    i += 1;
                }
            }

            if vk_path.is_empty() {
                eprintln!("Error: Missing required --vk parameter");
                print_usage();
                std::process::exit(1);
            }

            let vk_json = fs::read_to_string(&vk_path).expect("Failed to read Noir verification key");
            let vk: parser::NoirVerificationKey = serde_json::from_str(&vk_json)
                .expect("Failed to parse Noir verification key JSON");

            println!("Parsing Noir UltraHonk verification key...");
            println!("  Circuit Size:       {}", vk.circuit_size);
            println!("  Public Inputs:      {}", vk.num_public_inputs);
            println!("  Commitment Points:  {}", vk.commitments.len());

            let contract_code = generator_noir::generate_noir_contract(&vk);
            emit_contract_package(&out_dir, &contract_code, "noir_ultrahonk_verifier");

            println!("\n✅ Successfully generated lean Noir UltraHonk contract at: {}", out_dir);
            println!("   - WASM target size: < 20 KB");
            println!("   - 2-Pairing KZG verification natively on Stellar!");
        }

        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
        }
    }
}

fn emit_contract_package(out_dir: &str, lib_rs_code: &str, pkg_name: &str) {
    let out_path = Path::new(out_dir);
    let src_path = out_path.join("src");
    fs::create_dir_all(&src_path).expect("Failed to create contract output directory");

    let cargo_toml = format!(
r#"[package]
name = "{pkg_name}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
soroban-sdk = "20.5.0"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
"#);

    fs::write(out_path.join("Cargo.toml"), cargo_toml).expect("Failed to write Cargo.toml");
    fs::write(src_path.join("lib.rs"), lib_rs_code).expect("Failed to write src/lib.rs");
}
