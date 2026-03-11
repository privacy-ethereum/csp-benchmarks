use std::marker::PhantomData;

use crate::keccak256::u64target::{U64Target, xor_circuit};
use plonky2::{
    field::extension::Extendable,
    hash::hash_types::RichField,
    iop::{target::BoolTarget, witness::PartialWitness},
    plonk::circuit_builder::CircuitBuilder,
};

pub const ROUND_CONSTANTS: [u64; 24] = [
    1u64,
    0x8082u64,
    0x800000000000808au64,
    0x8000000080008000u64,
    0x808bu64,
    0x80000001u64,
    0x8000000080008081u64,
    0x8000000000008009u64,
    0x8au64,
    0x88u64,
    0x80008009u64,
    0x8000000au64,
    0x8000808bu64,
    0x800000000000008bu64,
    0x8000000000008089u64,
    0x8000000000008003u64,
    0x8000000000008002u64,
    0x8000000000000080u64,
    0x800au64,
    0x800000008000000au64,
    0x8000000080008081u64,
    0x8000000000008080u64,
    0x80000001u64,
    0x8000000080008008u64,
];
pub const ROTR: [usize; 25] = [
    0, 1, 62, 28, 27, 36, 44, 6, 55, 20, 3, 10, 43, 25, 39, 41, 45, 15, 21, 8, 18, 2, 61, 56, 14,
];

#[derive(Clone, Debug)]
pub struct KeccakTarget<F, const D: usize> {
    words: Vec<U64Target<F, D>>,
    _phantom: PhantomData<F>,
}

impl<F, const D: usize> KeccakTarget<F, D>
where
    F: RichField + Extendable<D>,
{
    pub fn new(builder: &mut CircuitBuilder<F, D>) -> Self {
        let mut result = vec![];
        for _ in 0..25 {
            result.push(U64Target::new(builder));
        }
        Self {
            words: result,
            _phantom: PhantomData,
        }
    }

    pub fn set_witness(&self, bits: Vec<bool>, pw: &mut PartialWitness<F>) {
        assert_eq!(bits.len(), 1600);
        for i in 0..25 {
            self.words[i].set_witness(bits[i * 64..(i + 1) * 64].to_vec(), pw);
        }
    }

    pub fn connect(&self, other: &Self, builder: &mut CircuitBuilder<F, D>) {
        for i in 0..25 {
            self.words[i].connect(&other.words[i], builder);
        }
    }

    pub fn from(bits: Vec<BoolTarget>) -> Self {
        let mut result = vec![];
        for i in 0..25 {
            result.push(U64Target::from(bits[i * 64..(i + 1) * 64].to_vec()));
        }
        Self {
            words: result,
            _phantom: PhantomData,
        }
    }

    // 641 gates
    pub fn keccak_round(&mut self, rc: u64, builder: &mut CircuitBuilder<F, D>) {
        // θ step
        let mut c = vec![];
        for x in 0..5 {
            let xor01 = self.words[x].xor(&self.words[x + 5], builder);
            let xor012 = xor01.xor(&self.words[x + 2 * 5], builder);
            let xor0123 = xor012.xor(&self.words[x + 3 * 5], builder);
            let xor01234 = xor0123.xor(&self.words[x + 4 * 5], builder);
            c.push(xor01234);
        }
        let mut d = vec![];
        for x in 0..5 {
            let rot_c = c[(x + 1) % 5].rotl(1);
            d.push(c[(x + 4) % 5].xor(&rot_c, builder));
        }
        for (x, dx) in d.iter().enumerate().take(5) {
            for y in 0..5 {
                self.words[x + y * 5] = self.words[x + y * 5].xor(dx, builder);
            }
        }
        // ρ and π steps
        let mut b_words: [Option<U64Target<F, D>>; 25] = [(); 25].map(|_| None);
        for x in 0..5 {
            for y in 0..5 {
                let rot_self = self.words[x + y * 5].rotl(ROTR[x + y * 5]);

                b_words[y + ((2 * x + 3 * y) % 5) * 5] = Some(rot_self);
            }
        }
        let b = KeccakTarget {
            words: b_words.into_iter().map(|x| x.unwrap()).collect(),
            _phantom: PhantomData,
        };

        // χ step
        for x in 0..5 {
            for y in 0..5 {
                // b.words[(x + 2) % 5 + y * 5] & !b.words[(x + 1) % 5 + y * 5]
                let and_not_b =
                    b.words[(x + 2) % 5 + y * 5].and_not(&b.words[(x + 1) % 5 + y * 5], builder);
                self.words[x + y * 5] = b.words[x + y * 5].xor(&and_not_b, builder);
            }
        }

        self.words[0] = self.words[0].xor_const(rc, builder);
    }

    pub fn keccakf(&self, builder: &mut CircuitBuilder<F, D>) -> Self {
        let mut result = self.clone();
        for round_constant in ROUND_CONSTANTS.into_iter().take(24) {
            result.keccak_round(round_constant, builder);
        }

        result
    }
}

pub fn keccak256_circuit<F, const D: usize>(
    input: Vec<BoolTarget>,
    builder: &mut CircuitBuilder<F, D>,
) -> Vec<BoolTarget>
where
    F: RichField + Extendable<D>,
{
    assert_eq!(input.len() % 8, 0); // input should be bytes.
    let block_size_in_bytes = 136; // in bytes
    let input_len_in_bytes = input.len() / 8;
    let num_blocks = input_len_in_bytes / block_size_in_bytes + 1;

    let mut padded = vec![];
    for _ in 0..block_size_in_bytes * 8 * num_blocks {
        padded.push(builder.add_virtual_bool_target_safe());
    }

    // register input
    for i in 0..input_len_in_bytes * 8 {
        builder.connect(padded[i].target, input[i].target);
    }

    // append 0x01 = 1000 0000 after the last input
    let true_target = builder.constant_bool(true);
    builder.connect(padded[input_len_in_bytes * 8].target, true_target.target);

    // pad 0s
    let false_target = builder.constant_bool(false);
    let last_index = padded.len() - 1;
    for padded_bit in padded
        .iter()
        .take(last_index)
        .skip(input_len_in_bytes * 8 + 1)
    {
        builder.connect(padded_bit.target, false_target.target);
    }

    // xor 0x80 = 0000 0001 with the last byte.
    // however the last bit is ensured to be 0, so just fill 1.
    builder.connect(padded[last_index].target, true_target.target);

    let mut m = KeccakTarget::new(builder);
    for i in 0..1600 {
        let word = i / 64;
        let bit = i % 64;
        builder.connect(m.words[word].bits[bit].target, false_target.target);
    }

    for i in 0..num_blocks {
        for j in 0..block_size_in_bytes * 8 {
            let word = j / 64;
            let bit = j % 64;
            let xor_t = xor_circuit(
                m.words[word].bits[bit],
                padded[i * block_size_in_bytes * 8 + j],
                builder,
            );
            m.words[word].bits[bit] = xor_t;
        }
        m = m.keccakf(builder);
    }

    let mut z = Vec::new();
    for i in 0..256 {
        let new_target = builder.add_virtual_bool_target_safe();
        let word = i / 64;
        let bit = i % 64;
        builder.connect(new_target.target, m.words[word].bits[bit].target);
        z.push(new_target);
    }
    z
}

pub fn array_to_bits_lsb(bytes: &[u8]) -> Vec<bool> {
    let mut ret = Vec::new();
    for byte in bytes {
        let mut n = *byte;
        for _ in 0..8 {
            ret.push((n & 1) == 1);
            n >>= 1;
        }
    }
    ret
}
