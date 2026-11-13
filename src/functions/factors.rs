//! Factorization functions — stubs.
//!
//! Trial-division based factoring. Manual implementation (no malachite equivalent).

use crate::numbers::BigNum;

/// Result of factoring out a prime `p` from `n`: `n = p^k * d`
pub struct FactorOut {
    /// The prime factor and its exponent.
    pub factor: Factor,
    /// The remaining cofactor `d` where `gcd(p^k, d) = 1`.
    pub cofactor: BigNum,
}

/// A prime factor `p` raised to power `k`.
pub struct Factor {
    /// The prime.
    pub p: BigNum,
    /// The exponent.
    pub k: u64,
}

/// Remove all occurrences of factor `f` from `n`.
/// Returns `(Factor { p: f, k: s }, d)` such that `n = f^s * d` and `gcd(f, d) != 0`.
pub fn factor_out(_n: &BigNum, _f: &BigNum) -> FactorOut {
    todo!("factor_out: trial division to remove factor f from n")
}

/// Full prime factorization of `n` via trial division.
pub fn factorize(_n: &BigNum) -> Vec<Factor> {
    todo!("factorize: trial division")
}

/// Check if `n` is square-free (no prime factor appears more than once).
pub fn square_free(_n: &BigNum) -> bool {
    todo!("square_free: trial division check")
}

/// Count the total and distinct prime factors of `n`.
pub fn prime_factor_count(_n: &BigNum) -> (u64, u64) {
    todo!("prime_factor_count: (total, distinct)")
}

