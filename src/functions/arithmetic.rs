//! Number-theoretic arithmetic functions — stubs.
//!
//! These depend on factorization; no malachite shortcut.

use crate::numbers::BigNum;

/// Unit function: `u(n) = 1` for all n.
pub fn u(_n: &BigNum) -> i64 {
    1
}

/// Kronecker delta: `I(n) = 1` if n == 1, else 0.
pub fn identity(n: &BigNum) -> i64 {
    if *n == BigNum::one() {
        1
    } else {
        0
    }
}

/// Identity function: `N(n) = n`.
pub fn big_n(n: &BigNum) -> BigNum {
    n.clone()
}

/// Euler's totient function φ(n).
pub fn phi(_n: &BigNum) -> BigNum {
    todo!("phi: requires factorize()")
}

/// Möbius function μ(n).
pub fn mu(_n: &BigNum) -> i64 {
    todo!("mu: requires prime_factor_count()")
}

/// Von Mangoldt function Λ(n).
pub fn mangoldt(_n: &BigNum) -> f64 {
    todo!("mangoldt: requires factorize()")
}

