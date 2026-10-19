use serde::{Deserialize, Serialize};

/// Point in G1 (affine coordinates x, y as hex strings or raw 32-byte arrays)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G1Point {
    pub x: String,
    pub y: String,
}

impl G1Point {
    pub fn to_bytes(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        let x_bytes = parse_hex_or_dec_32(&self.x);
        let y_bytes = parse_hex_or_dec_32(&self.y);
        out[..32].copy_from_slice(&x_bytes);
        out[32..].copy_from_slice(&y_bytes);
        out
    }
}

/// Point in G2 (affine coordinates x, y in Fp2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G2Point {
    pub x: [String; 2],
    pub y: [String; 2],
}

impl G2Point {
    pub fn to_bytes(&self) -> [u8; 128] {
        let mut out = [0u8; 128];
        let x0 = parse_hex_or_dec_32(&self.x[0]);
        let x1 = parse_hex_or_dec_32(&self.x[1]);
        let y0 = parse_hex_or_dec_32(&self.y[0]);
        let y1 = parse_hex_or_dec_32(&self.y[1]);
        out[0..32].copy_from_slice(&x0);
        out[32..64].copy_from_slice(&x1);
        out[64..96].copy_from_slice(&y0);
        out[96..128].copy_from_slice(&y1);
        out
    }
}

/// Standard Circom / SnarkJS Groth16 verification_key.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircomVerificationKey {
    pub protocol: String,
    pub curve: String,
    pub nPublic: usize,
    pub vk_alpha_1: [String; 3], // [x, y, 1]
    pub vk_beta_2: [[String; 2]; 3],
    pub vk_gamma_2: [[String; 2]; 3],
    pub vk_delta_2: [[String; 2]; 3],
    #[serde(rename = "IC")]
    pub ic: Vec<[String; 3]>,
}

/// Noir (UltraHonk / Barretenberg) Verification Key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoirVerificationKey {
    pub circuit_size: usize,
    pub num_public_inputs: usize,
    pub commitments: Vec<G1Point>,
    pub srs_g2: Option<G2Point>,
}

pub fn parse_hex_or_dec_32(s: &str) -> [u8; 32] {
    let s = s.trim();
    if s.starts_with("0x") || s.starts_with("0X") {
        let clean = &s[2..];
        let mut padded = clean.to_string();
        if padded.len() < 64 {
            padded = format!("{:0>64}", padded);
        }
        let decoded = hex::decode(&padded).unwrap_or_else(|_| vec![0u8; 32]);
        let mut res = [0u8; 32];
        let len = decoded.len().min(32);
        res[32 - len..].copy_from_slice(&decoded[decoded.len() - len..]);
        res
    } else {
        // Decimal string to 32-byte big-endian
        let mut val = [0u8; 32];
        for ch in s.bytes() {
            if ch >= b'0' && ch <= b'9' {
                let digit = ch - b'0';
                // val = val * 10 + digit
                let mut carry = digit as u16;
                for byte in val.iter_mut().rev() {
                    let acc = (*byte as u16) * 10 + carry;
                    *byte = (acc & 0xff) as u8;
                    carry = acc >> 8;
                }
            }
        }
        val
    }
}
