//! Primality testing and prime generation — stubs.
//!
//! Uses malachite `mod_pow` for Miller-Rabin internals; rest is manual.

use crate::numbers::BigNum;

/// Miller-Rabin deterministic witness bases for n < 3,317,044,064,679,887,385,961,981.
pub const MILLER_RABIN_K_BASES: [u64; 13] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
];

/// Test whether `n` is (probably) prime using Miller-Rabin.
pub fn is_prime(_n: &BigNum) -> bool {
    todo!("is_prime: delegates to miller_rabin_primality")
}

/// Miller-Rabin primality test with given witness bases.
pub fn miller_rabin_primality(_n: &BigNum, _bases: &[u64]) -> bool {
    todo!("miller_rabin_primality: uses mod_pow from malachite")
}

/// Sieve of Eratosthenes up to `n`.
pub fn sieve_of_eratosthenes(_n: u64) -> Vec<u64> {
    todo!("sieve_of_eratosthenes")
}

/// All primes up to `n`.
pub fn primes(n: u64) -> Vec<u64> {
    sieve_of_eratosthenes(n)
}

/// Prime counting function: number of primes ≤ n.
pub fn pi(n: u64) -> usize {
    sieve_of_eratosthenes(n).len()
}

/// Check if `a` and `b` are coprime (`gcd(a,b) == 1`).
pub fn is_relatively_prime(a: &BigNum, b: &BigNum) -> bool {
    crate::functions::elementary::gcd(a, b) == BigNum::one()
}

