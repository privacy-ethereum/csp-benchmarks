#![cfg(test)]
//! Lattice hint for the 4-dimensional fake-GLV scalar multiplication.
//!
//! Test-only. The circuit derives the hint itself, so nothing on the proving
//! path calls this; it stays as the independent implementation the derivation
//! is checked against.
//!
//! The circuit does not compute `[u2]Q`; it *verifies* it. The prover supplies
//! `(v0, v1, v2, v3)` with
//!
//! ```text
//! v0 + v1*L - u2*v2 - u2*L*v3 == 0  (mod n),   L = lambda
//! ```
//!
//! and the circuit checks that congruence plus a 4-base Straus loop over the
//! four ~64-bit magnitudes. Finding a short `v` needs lattice reduction, which
//! cannot be expressed in circom's witness language, so it happens here — the
//! same pattern gnark uses for its hints. Soundness does not rest on this code
//! being correct: a wrong hint simply fails to produce a witness.
//!
//! The reduction is de Weger / Cohen Alg. 2.6.7 (integer LLL): Gram
//! determinants `d[i]` and scaled coefficients `lam[i][j] = d[j+1]*mu[i][j]`
//! are updated incrementally, so every division stays exact and no rationals
//! are needed. `delta = 99/100`, size reduction over all `j < k` before the
//! Lovasz test, rounding half up — identical to the JavaScript reference in
//! `scripts/glv4_lib.js`, which is what the circuit was tested against.

use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use std::sync::LazyLock;

/// secp256k1 group order.
pub static ORDER_N: LazyLock<BigInt> =
    LazyLock::new(|| parse_hex("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141"));

/// Eigenvalue of the endomorphism `phi(x, y) = (beta*x, y)`: `phi(P) = [lambda]P`.
pub static LAMBDA: LazyLock<BigInt> =
    LazyLock::new(|| parse_hex("5363ad4cc05c30e0a5261c028812645a122e22ea20816678df02967c1b23bd72"));

fn parse_hex(s: &str) -> BigInt {
    BigInt::parse_bytes(s.as_bytes(), 16).expect("valid hex constant")
}

/// Euclidean remainder: result is always in `[0, m)`.
fn rem_euclid(a: &BigInt, m: &BigInt) -> BigInt {
    let r = a % m;
    if r.is_negative() { r + m } else { r }
}

/// Floor division, i.e. rounding towards negative infinity.
///
/// `BigInt` division truncates towards zero, which rounds the wrong way for
/// negative quotients and would make the size reduction disagree with the
/// reference implementation.
fn floor_div(a: &BigInt, b: &BigInt) -> BigInt {
    let q = a / b;
    if (a % b).is_zero() || a.is_negative() == b.is_negative() {
        q
    } else {
        q - 1
    }
}

fn dot(u: &[BigInt], v: &[BigInt]) -> BigInt {
    u.iter().zip(v).map(|(a, b)| a * b).sum()
}

/// Integer LLL. Returns the reduced basis, same row count as the input.
// The index loops read one row and write another (`b[j][t]` into `b[k][t]`),
// which `needless_range_loop` does not account for: its iterator rewrite walks
// the rows of the matrix, not the columns of one row. The loops also have to
// stay step for step identical to the reference implementation they are
// checked against.
#[allow(clippy::needless_range_loop)]
pub fn lll_int(basis_in: &[Vec<BigInt>]) -> Vec<Vec<BigInt>> {
    let dim = basis_in.len();
    let mut b: Vec<Vec<BigInt>> = basis_in.to_vec();

    // d[i] = det Gram(b_0..b_{i-1}), d[0] = 1;  lam[i][j] = d[j+1] * mu[i][j]
    let mut d: Vec<BigInt> = vec![BigInt::zero(); dim + 1];
    d[0] = BigInt::one();
    let mut lam: Vec<Vec<BigInt>> = vec![vec![BigInt::zero(); dim]; dim];

    for i in 0..dim {
        for j in 0..=i {
            let mut u = dot(&b[i], &b[j]);
            for t in 0..j {
                u = (&d[t + 1] * &u - &lam[i][t] * &lam[j][t]) / &d[t];
            }
            if j < i {
                lam[i][j] = u;
            } else {
                d[i + 1] = u;
            }
        }
        assert!(d[i + 1].is_positive(), "LLL: linearly dependent basis");
    }

    let mut k = 1usize;
    let mut guard = 0u32;
    while k < dim {
        guard += 1;
        assert!(guard <= 100_000, "LLL does not converge");

        // size reduction against every previous vector, highest index first
        for j in (0..k).rev() {
            let den = d[j + 1].clone();
            let q = floor_div(&(&lam[k][j] * 2 + &den), &(&den * 2));
            if q.is_zero() {
                continue;
            }
            for t in 0..dim {
                let delta = &q * &b[j][t];
                b[k][t] -= delta;
            }
            lam[k][j] -= &q * &den;
            for i in 0..j {
                let delta = &q * &lam[j][i];
                lam[k][i] -= delta;
            }
        }

        // Lovasz with delta = 99/100:
        //   100*d[k+1]*d[k-1] >= 99*d[k]^2 - 100*lam[k][k-1]^2
        let lhs = &d[k + 1] * &d[k - 1] * 100;
        let rhs = &d[k] * &d[k] * 99 - &lam[k][k - 1] * &lam[k][k - 1] * 100;
        if lhs >= rhs {
            k += 1;
        } else {
            swap_step(&mut b, &mut lam, &mut d, dim, k);
            k = k.saturating_sub(1).max(1);
        }
    }

    b
}

// Same reason as `lll_int`: the loops below swap entries between two rows of
// `lam`, so the loop variable is not a plain iteration over one of them.
#[allow(clippy::needless_range_loop)]
fn swap_step(
    b: &mut [Vec<BigInt>],
    lam: &mut [Vec<BigInt>],
    d: &mut [BigInt],
    dim: usize,
    k: usize,
) {
    b.swap(k, k - 1);
    for j in 0..k - 1 {
        let t = lam[k][j].clone();
        lam[k][j] = lam[k - 1][j].clone();
        lam[k - 1][j] = t;
    }
    let l0 = lam[k][k - 1].clone();
    let big_b = (&d[k - 1] * &d[k + 1] + &l0 * &l0) / &d[k];
    for i in k + 1..dim {
        let t = lam[i][k].clone();
        lam[i][k] = (&d[k + 1] * &lam[i][k - 1] - &l0 * &t) / &d[k];
        lam[i][k - 1] = (&big_b * &t + &l0 * &lam[i][k]) / &d[k + 1];
    }
    d[k] = big_b;
}

/// Of the reduced basis, the non-zero vector with the smallest largest
/// coordinate.
///
/// Not the shortest vector by norm: the circuit pays for the *bit length* of
/// the biggest scalar (it runs one Straus step per bit), so `max|coord|` is the
/// cost function that matters.
pub fn shortest_by_max_coord(reduced: &[Vec<BigInt>]) -> Vec<BigInt> {
    reduced
        .iter()
        .filter(|v| v.iter().any(|x| !x.is_zero()))
        .min_by_key(|v| v.iter().map(|x| x.abs()).max().expect("non-empty vector"))
        .expect("reduced basis has a non-zero vector")
        .clone()
}

/// Basis of `{ x in Z^4 : x0 + x1*L - s*x2 - s*L*x3 == 0 (mod n) }`, det = n.
pub fn lattice_basis(s: &BigInt) -> Vec<Vec<BigInt>> {
    let n = &*ORDER_N;
    let l = &*LAMBDA;
    let zero = BigInt::zero;
    vec![
        vec![n.clone(), zero(), zero(), zero()],
        vec![rem_euclid(&-l.clone(), n), BigInt::one(), zero(), zero()],
        vec![rem_euclid(s, n), zero(), BigInt::one(), zero()],
        vec![rem_euclid(&(s * l), n), zero(), zero(), BigInt::one()],
    ]
}

/// The lattice hint for scalar `s`: four signed values, each ~64 bits.
pub fn decompose4(s: &BigInt) -> Vec<BigInt> {
    shortest_by_max_coord(&lll_int(&lattice_basis(s)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::Num;

    /// Vectors produced by `scripts/glv4_lib.js`, the implementation the
    /// circuit was tested against (`test_ecdsa4.js`, 21/21 on the 503,280
    /// build). Equality here is the whole point: a decomposition that is short
    /// but *different* still fails, because the circuit's Straus loop is
    /// deterministic in the hint.
    const REFERENCE: &[(&str, [&str; 4])] = &[
        (
            "15041928259909583962564091140219451443772478959334398733990144463954766665407",
            [
                "4466359167668580647",
                "8523695487348571436",
                "-4252393916719459165",
                "5175486731732238870",
            ],
        ),
        (
            "101968455937223092386265578421733251027410941584093652157636843492449321885779",
            [
                "7753311262285893544",
                "-6035487210137233946",
                "-3898736396773334958",
                "-13336194031741671831",
            ],
        ),
    ];

    fn dec(s: &str) -> BigInt {
        BigInt::from_str_radix(s, 10).unwrap()
    }

    #[test]
    fn matches_javascript_reference() {
        for (s, expected) in REFERENCE {
            let v = decompose4(&dec(s));
            let got: Vec<String> = v.iter().map(|x| x.to_string()).collect();
            assert_eq!(got, expected.to_vec(), "decompose4({s})");
        }
    }

    #[test]
    fn satisfies_the_lattice_relation() {
        for (s, _) in REFERENCE {
            let s = dec(s);
            let v = decompose4(&s);
            let rel = &v[0] + &v[1] * &*LAMBDA - &s * &v[2] - &s * &*LAMBDA * &v[3];
            assert!(rem_euclid(&rel, &ORDER_N).is_zero(), "relation for {s}");
        }
    }

    #[test]
    fn magnitudes_fit_in_64_bits() {
        for (s, _) in REFERENCE {
            let v = decompose4(&dec(s));
            for x in &v {
                assert!(x.abs().bits() <= 64, "{x} needs {} bits", x.abs().bits());
            }
        }
    }

    #[test]
    fn floor_div_rounds_towards_negative_infinity() {
        let cases = [
            (7, 2, 3),
            (-7, 2, -4),
            (7, -2, -4),
            (-7, -2, 3),
            (-6, 2, -3),
        ];
        for (a, b, want) in cases {
            assert_eq!(
                floor_div(&BigInt::from(a), &BigInt::from(b)),
                BigInt::from(want),
                "floor_div({a}, {b})"
            );
        }
    }
}
