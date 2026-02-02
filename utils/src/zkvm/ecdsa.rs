use crate::zkvm::instance::ProofArtifacts;
use crate::zkvm::traits::PreparedBenchmark;
use ere_zkvm_interface::{Input, Proof, ProofKind, PublicValues, zkVM};

/// Benchmark name for ECDSA programs.
pub const ECDSA_BENCH: &str = "ecdsa";

/// ECDSA signature size in bytes (r || s).
pub const SIGNATURE_SIZE: usize = 64;

/// Encoded public key size in uncompressed SEC1 format.
pub const ENCODED_PUBLIC_KEY_SIZE: usize = 65;

/// secp256k1 coordinate size in bytes.
pub const COORDINATE_SIZE: usize = 32;

/// Encodes public key (x, y) into uncompressed SEC1 format: [0x04, x, y].
pub fn encode_public_key(pub_key_x: &[u8], pub_key_y: &[u8]) -> Result<Vec<u8>, &'static str> {
    if pub_key_x.len() != COORDINATE_SIZE {
        return Err("Public key X coordinate must be 32 bytes");
    }
    if pub_key_y.len() != COORDINATE_SIZE {
        return Err("Public key Y coordinate must be 32 bytes");
    }

    let mut encoded = Vec::with_capacity(ENCODED_PUBLIC_KEY_SIZE);
    encoded.push(0x04);
    encoded.extend_from_slice(pub_key_x);
    encoded.extend_from_slice(pub_key_y);
    Ok(encoded)
}

/// Common preparation data for zkVM ECDSA benchmarks.
pub struct PreparedEcdsa<V> {
    vm: V,
    input: Input,
    compiled_size: usize,
    expected_public_key: Option<(Vec<u8>, Vec<u8>)>,
    expected_message: Option<Vec<u8>>,
}

impl<V> PreparedEcdsa<V> {
    pub fn new(vm: V, input: Input, compiled_size: usize) -> Self {
        Self {
            vm,
            input,
            compiled_size,
            expected_public_key: None,
            expected_message: None,
        }
    }

    pub fn with_expected_values(
        vm: V,
        input: Input,
        compiled_size: usize,
        expected_public_key: (Vec<u8>, Vec<u8>),
        expected_message: Vec<u8>,
    ) -> Self {
        Self {
            vm,
            input,
            compiled_size,
            expected_public_key: Some(expected_public_key),
            expected_message: Some(expected_message),
        }
    }

    pub fn compiled_size(&self) -> usize {
        self.compiled_size
    }

    pub fn expected_public_key(&self) -> Option<&(Vec<u8>, Vec<u8>)> {
        self.expected_public_key.as_ref()
    }

    pub fn expected_message(&self) -> Option<&[u8]> {
        self.expected_message.as_deref()
    }

    pub fn vm(&self) -> &V {
        &self.vm
    }

    pub fn input(&self) -> &Input {
        &self.input
    }
}

impl<V> PreparedEcdsa<V>
where
    V: zkVM,
{
    pub fn prove(&self) -> Result<ProofArtifacts, anyhow::Error> {
        let (public_values, proof, report) = self.vm.prove(&self.input, ProofKind::default())?;
        Ok(ProofArtifacts::new(public_values, proof, report))
    }

    pub fn verify(&self, proof: &Proof) -> Result<PublicValues, anyhow::Error> {
        self.vm.verify(proof)
    }

    pub fn verify_with_expected(&self, proof: &ProofArtifacts) -> Result<(), anyhow::Error> {
        let public_values = self.vm.verify(&proof.proof)?;

        if public_values != proof.public_values {
            return Err(anyhow::anyhow!("public values mismatch"));
        }

        // Validate expected values if provided
        if let (Some(expected_key), Some(expected_msg)) =
            (&self.expected_public_key, &self.expected_message)
        {
            // The guest commits (encoded_verifying_key, message) as a bincode tuple
            use bincode::Options;
            let (committed_key, committed_msg): (Vec<u8>, Vec<u8>) = bincode::options()
                .deserialize(&public_values)
                .map_err(|_| anyhow::anyhow!("failed to deserialize public values"))?;

            // Reconstruct expected encoded key from x,y coordinates
            let expected_encoded = encode_public_key(&expected_key.0, &expected_key.1)
                .map_err(|e| anyhow::anyhow!("{}", e))?;

            if committed_key != expected_encoded {
                return Err(anyhow::anyhow!("public key mismatch"));
            }

            if committed_msg != *expected_msg {
                return Err(anyhow::anyhow!("message mismatch"));
            }
        }

        Ok(())
    }

    pub fn execution_cycles(&self) -> Result<u64, anyhow::Error> {
        let (_, report) = self.vm.execute(&self.input)?;
        Ok(report.total_num_cycles)
    }
}

impl<V: zkVM> PreparedBenchmark for PreparedEcdsa<V> {
    type VM = V;

    fn compiled_size(&self) -> usize {
        self.compiled_size
    }

    fn execution_cycles(&self) -> Result<u64, anyhow::Error> {
        PreparedEcdsa::execution_cycles(self)
    }

    fn prove(&self) -> Result<ProofArtifacts, anyhow::Error> {
        PreparedEcdsa::prove(self)
    }

    fn vm(&self) -> &Self::VM {
        &self.vm
    }

    fn input(&self) -> &Input {
        &self.input
    }
}

/// Builds zkVM input for ECDSA verification: (encoded_verifying_key, message, signature).
pub fn build_ecdsa_input(
    encoded_verifying_key: Vec<u8>,
    message: Vec<u8>,
    signature: Vec<u8>,
) -> Result<Input, &'static str> {
    use bincode::Options;

    if encoded_verifying_key.len() != ENCODED_PUBLIC_KEY_SIZE {
        return Err("Encoded verifying key must be 65 bytes");
    }
    if signature.len() != SIGNATURE_SIZE {
        return Err("Signature must be 64 bytes");
    }

    let data = (encoded_verifying_key, message, signature);
    let serialized = bincode::options()
        .serialize(&data)
        .expect("failed to serialize ECDSA input");

    Ok(Input::new().with_stdin(serialized))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_public_key_format() {
        let x = vec![1u8; 32];
        let y = vec![2u8; 32];
        let encoded = encode_public_key(&x, &y).unwrap();

        assert_eq!(encoded.len(), 65);
        assert_eq!(encoded[0], 0x04);
        assert_eq!(&encoded[1..33], &x[..]);
        assert_eq!(&encoded[33..65], &y[..]);
    }

    #[test]
    fn test_encode_public_key_validates_x_size() {
        let result = encode_public_key(&[1u8; 31], &[2u8; 32]);
        assert_eq!(result, Err("Public key X coordinate must be 32 bytes"));
    }

    #[test]
    fn test_encode_public_key_validates_y_size() {
        let result = encode_public_key(&[1u8; 32], &[2u8; 33]);
        assert_eq!(result, Err("Public key Y coordinate must be 32 bytes"));
    }

    #[test]
    fn test_build_ecdsa_input_with_valid_sizes() {
        let key = vec![4u8; 65];
        let msg = vec![5u8; 32];
        let sig = vec![6u8; 64];

        let result = build_ecdsa_input(key, msg, sig);
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_ecdsa_input_validates_key_size() {
        let result = build_ecdsa_input(vec![4u8; 64], vec![5u8; 32], vec![6u8; 64]);
        assert_eq!(
            result.unwrap_err(),
            "Encoded verifying key must be 65 bytes"
        );
    }

    #[test]
    fn test_build_ecdsa_input_validates_signature_size() {
        let result = build_ecdsa_input(vec![4u8; 65], vec![5u8; 32], vec![6u8; 63]);
        assert_eq!(result.unwrap_err(), "Signature must be 64 bytes");
    }
}
