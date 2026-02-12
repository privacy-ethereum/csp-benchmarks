/*
 * Copyright (C) 2023-2025 Ligero, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// https://github.com/ligeroinc/ligero-prover/blob/main/sdk/rust/src/poseidon2.rs
// Adapted to use ark-ff/ark-bn254

//! Poseidon2 Hash Function for Ligetron
//!
//! ## Algorithm Details
//!
//! Poseidon2 uses a t=2 state size with:
//! - **External MDS Matrix**: [2, 1; 1, 2]
//! - **Internal MDS Matrix**: [2, 1; 1, 3]
//! - **Round Structure**: 8 full rounds, 56 partial rounds
//! - **S-box**: x^5 power function
//! ```
//!
//! ## Performance Considerations
//!
//! - **Byte Processing**: Data is processed in 31-byte chunks (field element size)
//! - **Padding**: Automatic padding applied to incomplete chunks

use ark_bn254::Fr;
use ark_ff::{Field, PrimeField};

use super::poseidon2_constant::{
    POSEIDON2_BN254_RF, POSEIDON2_BN254_RP, POSEIDON2_BN254_T, POSEIDON2_T2_RC_STR,
};

fn fr_from_hex_str(s: &str) -> Fr {
    Fr::from_be_bytes_mod_order(&hex::decode(s.strip_prefix("0x").unwrap()).unwrap())
}

/// Constants for Poseidon2 BN254 with t=2 (state size 2)
pub struct Poseidon2Params {
    pub r_f: usize, // Full rounds
    pub r_p: usize, // Partial rounds
    pub t: usize,   // State size
}

impl Default for Poseidon2Params {
    fn default() -> Self {
        Poseidon2Params {
            r_f: POSEIDON2_BN254_RF,
            r_p: POSEIDON2_BN254_RP,
            t: POSEIDON2_BN254_T,
        }
    }
}

/// Poseidon2 hash context for BN254 field elements (t=2)
pub struct Poseidon2Context {
    state: [Fr; 2],
    params: Poseidon2Params,
    buffer: Vec<u8>,
    buffer_len: usize,
    temp: Fr,
    rc: Vec<Fr>,
}

#[allow(clippy::new_without_default)]
impl Poseidon2Context {
    pub fn new() -> Self {
        let rc = POSEIDON2_T2_RC_STR
            .iter()
            .map(|&s| fr_from_hex_str(s))
            .collect();

        Poseidon2Context {
            state: [Fr::from(0u64), Fr::from(0u64)],
            params: Poseidon2Params::default(),
            buffer: vec![0u8; 31],
            buffer_len: 0,
            temp: Fr::from(0u64),
            rc,
        }
    }

    // resets the internal context state
    pub fn digest_init(&mut self) {
        self.state[0] = Fr::from(0u64);
        self.state[1] = Fr::from(0u64);
        self.buffer_len = 0;
        for i in 0..31 {
            self.buffer[i] = 0;
        }
    }

    pub fn digest_update(&mut self, data: &Fr) {
        self.state[0] += data;
        self.permute();
    }

    pub fn digest_update_bytes(&mut self, data: &[u8]) {
        let mut offset = 0;
        let mut remaining = data.len();

        // Process 31-byte chunks
        while remaining >= 31 {
            let chunk = &data[offset..offset + 31];
            self.temp = Fr::from_be_bytes_mod_order(chunk);
            self.state[0] += self.temp;
            self.permute();
            offset += 31;
            remaining -= 31;
        }

        // Handle remaining bytes
        for &byte in &data[offset..] {
            self.buffer[self.buffer_len] = byte;
            self.buffer_len += 1;

            if self.buffer_len >= 31 {
                self.temp = Fr::from_be_bytes_mod_order(&self.buffer[..31]);
                self.state[0] += self.temp;
                self.permute();
                self.buffer_len = 0;
            }
        }
    }

    /// Finalize the hash computation and get the result
    pub fn digest_final(&mut self) -> Fr {
        self.buffer[self.buffer_len] = 0x80;
        self.buffer_len += 1;

        while self.buffer_len < 31 {
            self.buffer[self.buffer_len] = 0;
            self.buffer_len += 1;
        }

        self.temp = Fr::from_be_bytes_mod_order(&self.buffer[..31]);
        self.state[0] += self.temp;
        self.permute();

        self.state[0]
    }

    /// Internal permutation function for Poseidon2
    fn permute(&mut self) {
        // External MDS multiplication
        self.multiply_external_mds();

        let mut round = 0;

        // First half of full rounds
        for _ in 0..4 {
            self.add_round_constants(round);
            self.sbox_full();
            self.multiply_external_mds();
            round += 1;
        }

        // Partial rounds
        for _ in 0..self.params.r_p {
            self.add_round_constants_partial(round);
            self.sbox_partial();
            self.multiply_internal_mds();
            round += 1;
        }

        // Second half of full rounds
        for _ in 0..4 {
            self.add_round_constants(round);
            self.sbox_full();
            self.multiply_external_mds();
            round += 1;
        }
    }

    /// Add round constants to the state (full rounds)
    fn add_round_constants(&mut self, round: usize) {
        self.state[0] += self.rc[round * 2];
        self.state[1] += self.rc[round * 2 + 1];
    }

    /// Add round constants to the state (partial rounds - only first element)
    fn add_round_constants_partial(&mut self, round: usize) {
        self.state[0] += self.rc[round * 2];
    }

    /// Apply S-box (x^5) to all elements
    fn sbox_full(&mut self) {
        self.state[0] = Self::pow5(self.state[0]);
        self.state[1] = Self::pow5(self.state[1]);
    }

    /// Apply S-box (x^5) to first element only
    fn sbox_partial(&mut self) {
        self.state[0] = Self::pow5(self.state[0]);
    }

    /// Compute x^5 for field element
    fn pow5(x: Fr) -> Fr {
        let x2 = x.square(); // x^2
        x2.square() * x // x^4 * x = x^5
    }

    /// External MDS matrix multiplication for t=2
    /// External MDS = [2, 1]
    ///                [1, 2]
    fn multiply_external_mds(&mut self) {
        self.temp = self.state[0] + self.state[1];
        self.state[0] += self.temp;
        self.state[1] += self.temp;
    }

    /// Internal MDS matrix multiplication for t=2
    /// Internal MDS = [2, 1]
    ///                [1, 3]
    fn multiply_internal_mds(&mut self) {
        self.temp = self.state[0] + self.state[1];
        self.state[0] += self.temp;
        self.temp += self.state[1];
        self.state[1] += self.temp;
    }
}

/// Poseidon2 hash context for vectorized BN254 field elements (t=2)
/// NOTE: In the original SDK this uses VBn254Fr for SIMD-style vectorized operations.
/// Here we use scalar Fr since we only need reference hashing, not prover performance.
pub struct VPoseidon2Context {
    state: [Fr; 2],
    params: Poseidon2Params,
    buffer: Vec<u8>,
    buffer_len: usize,
    temp: Fr,
    rc: Vec<Fr>,
}

#[allow(clippy::new_without_default)]
impl VPoseidon2Context {
    pub fn new() -> Self {
        let rc = POSEIDON2_T2_RC_STR
            .iter()
            .map(|&s| fr_from_hex_str(s))
            .collect();

        VPoseidon2Context {
            state: [Fr::from(0u64), Fr::from(0u64)],
            params: Poseidon2Params::default(),
            buffer: vec![0u8; 31],
            buffer_len: 0,
            temp: Fr::from(0u64),
            rc,
        }
    }

    // resets the internal context state
    pub fn digest_init(&mut self) {
        self.state[0] = Fr::from(0u64);
        self.state[1] = Fr::from(0u64);
        self.buffer_len = 0;
        for i in 0..31 {
            self.buffer[i] = 0;
        }
    }

    pub fn digest_update(&mut self, data: &Fr) {
        // Add the data to state[0] and perform permutation
        self.state[0] += data;
        self.permute();
    }

    pub fn digest_update_bytes(&mut self, data: &[u8]) {
        let mut offset = 0;
        let mut remaining = data.len();

        // Process 31-byte chunks
        while remaining >= 31 {
            let chunk = &data[offset..offset + 31];
            self.temp = Fr::from_be_bytes_mod_order(chunk);
            self.state[0] += self.temp;
            self.permute();
            offset += 31;
            remaining -= 31;
        }

        // Handle remaining bytes
        for &byte in &data[offset..] {
            self.buffer[self.buffer_len] = byte;
            self.buffer_len += 1;

            if self.buffer_len >= 31 {
                self.temp = Fr::from_be_bytes_mod_order(&self.buffer[..31]);
                self.state[0] += self.temp;
                self.permute();
                self.buffer_len = 0;
            }
        }
    }

    /// Finalize the hash computation and get the result
    pub fn digest_final(&mut self) -> Fr {
        self.buffer[self.buffer_len] = 0x80;
        self.buffer_len += 1;

        while self.buffer_len < 31 {
            self.buffer[self.buffer_len] = 0;
            self.buffer_len += 1;
        }

        self.temp = Fr::from_be_bytes_mod_order(&self.buffer[..31]);
        self.state[0] += self.temp;
        self.permute();

        self.state[0]
    }

    fn permute(&mut self) {
        // External MDS multiplication
        self.multiply_external_mds();

        let mut round = 0;

        // First half of full rounds
        for _ in 0..4 {
            self.add_round_constants(round);
            self.sbox_full();
            self.multiply_external_mds();
            round += 1;
        }

        // Partial rounds
        for _ in 0..self.params.r_p {
            self.add_round_constants_partial(round);
            self.sbox_partial();
            self.multiply_internal_mds();
            round += 1;
        }

        // Second half of full rounds
        for _ in 0..4 {
            self.add_round_constants(round);
            self.sbox_full();
            self.multiply_external_mds();
            round += 1;
        }
    }

    fn add_round_constants(&mut self, round: usize) {
        self.state[0] += self.rc[round * 2];
        self.state[1] += self.rc[round * 2 + 1];
    }

    fn add_round_constants_partial(&mut self, round: usize) {
        self.state[0] += self.rc[round * 2];
    }

    /// Apply S-box (x^5) to all elements
    fn sbox_full(&mut self) {
        self.state[0] = Self::pow5(self.state[0]);
        self.state[1] = Self::pow5(self.state[1]);
    }

    /// Apply S-box (x^5) to first element only
    fn sbox_partial(&mut self) {
        self.state[0] = Self::pow5(self.state[0]);
    }

    /// Compute x^5 for vectorized field element
    fn pow5(x: Fr) -> Fr {
        let x2 = x.square(); // x^2
        x2.square() * x // x^4 * x = x^5
    }

    /// External MDS matrix multiplication for t=2
    fn multiply_external_mds(&mut self) {
        self.temp = self.state[0] + self.state[1];
        self.state[0] += self.temp;
        self.state[1] += self.temp;
    }

    /// Internal MDS matrix multiplication for t=2
    fn multiply_internal_mds(&mut self) {
        self.temp = self.state[0] + self.state[1];
        self.state[0] += self.temp;
        self.temp += self.state[1];
        self.state[1] += self.temp;
    }
}

/// Convenience function to compute Poseidon2 hash from field elements
pub fn poseidon2_hash(inputs: &[Fr]) -> Fr {
    let mut ctx = Poseidon2Context::new();

    for input in inputs {
        ctx.digest_update(input);
    }

    ctx.digest_final()
}

/// Convenience function to compute Poseidon2 hash from bytes
pub fn poseidon2_hash_bytes(data: &[u8]) -> Fr {
    let mut ctx = Poseidon2Context::new();
    ctx.digest_update_bytes(data);
    ctx.digest_final()
}

/// Convenience function to compute vectorized Poseidon2 hash from field elements
pub fn vposeidon2_hash(inputs: &[Fr]) -> Fr {
    let mut ctx = VPoseidon2Context::new();

    for input in inputs {
        ctx.digest_update(input);
    }

    ctx.digest_final()
}

/// Convenience function to compute vectorized Poseidon2 hash from bytes
pub fn vposeidon2_hash_bytes(data: &[u8]) -> Fr {
    let mut ctx = VPoseidon2Context::new();
    ctx.digest_update_bytes(data);
    ctx.digest_final()
}
