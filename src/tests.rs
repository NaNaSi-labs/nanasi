#[cfg(test)]
mod tests {
    use crate::parser::*;
    use crate::generator_circom::generate_circom_contract;
    use crate::generator_noir::generate_noir_contract;

    #[test]
    fn test_parse_hex_or_dec_32() {
        // Hex parsing with 0x
        let hex_val = "0x000000000000000000000000000000000000000000000000000000000000002a";
        let parsed = parse_hex_or_dec_32(hex_val);
        assert_eq!(parsed[31], 0x2a);
        assert_eq!(parsed[0], 0x00);

        // Decimal parsing: 42
        let dec_val = "42";
        let parsed_dec = parse_hex_or_dec_32(dec_val);
        assert_eq!(parsed_dec[31], 42);
        assert_eq!(parsed_dec[0], 0);

        // Decimal parsing: 256
        let dec_256 = "256";
        let parsed_256 = parse_hex_or_dec_32(dec_256);
        assert_eq!(parsed_256[31], 0);
        assert_eq!(parsed_256[30], 1);
    }

    #[test]
    fn test_circom_vk_serialization() {
        let json_data = include_str!("../sample_circom_vk.json");
        let vk: CircomVerificationKey = serde_json::from_str(json_data).expect("Failed to deserialize Circom VK");

        assert_eq!(vk.protocol, "groth16");
        assert_eq!(vk.curve, "bn128");
        assert_eq!(vk.nPublic, 2);
        assert_eq!(vk.ic.len(), 3);

        let contract_code = generate_circom_contract(&vk);
        assert!(contract_code.contains("pub struct CircomGroth16Verifier"));
        assert!(contract_code.contains("bn.g1_msm"));
        assert!(contract_code.contains("bn.pairing_check"));
        assert!(contract_code.contains("pub const NUM_PUBLIC_INPUTS: usize = 2;"));
    }

    #[test]
    fn test_noir_vk_serialization() {
        let json_data = include_str!("../sample_noir_vk.json");
        let vk: NoirVerificationKey = serde_json::from_str(json_data).expect("Failed to deserialize Noir VK");

        assert_eq!(vk.circuit_size, 2048);
        assert_eq!(vk.num_public_inputs, 2);
        assert_eq!(vk.commitments.len(), 3);

        let contract_code = generate_noir_contract(&vk);
        assert!(contract_code.contains("pub struct NoirUltraHonkVerifier"));
        assert!(contract_code.contains("bn.g1_msm"));
        assert!(contract_code.contains("bn.pairing_check"));
        assert!(contract_code.contains("pub const NUM_PUBLIC_INPUTS: usize = 2;"));
        assert!(contract_code.contains("pub const CIRCUIT_SIZE: usize = 2048;"));
    }
}
