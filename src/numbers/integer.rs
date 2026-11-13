//! Arbitrary-precision signed integer — thin wrapper over `malachite::Integer`.

use malachite::Integer;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use std::str::FromStr;

/// Arbitrary-precision signed integer.
///
/// Intentionally lean — number-theoretic operations (gcd, lcm, factorial, etc.)
/// live in [`crate::functions::elementary`] and operate on the inner `Integer` directly.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BigNum(pub Integer);

impl BigNum {
    pub fn new(val: i64) -> Self {
        BigNum(Integer::from(val))
    }

    pub fn from_u64(val: u64) -> Self {
        BigNum(Integer::from(val))
    }

    pub fn from_integer(i: Integer) -> Self {
        BigNum(i)
    }

    pub fn zero() -> Self {
        BigNum(Integer::from(0))
    }

    pub fn one() -> Self {
        BigNum(Integer::from(1))
    }

    pub fn inner(&self) -> &Integer {
        &self.0
    }

    pub fn into_inner(self) -> Integer {
        self.0
    }

    pub fn abs(&self) -> Self {
        BigNum(Integer::from((&self.0).unsigned_abs_ref()))
    }
}

// ── Display / FromStr ──────────────────────────────────────────

impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for BigNum {
    type Err = crate::error::RMathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Integer::from_str(s)
            .map(BigNum)
            .map_err(|_| crate::error::RMathError::InvalidNumberFormat(s.to_string()))
    }
}

// ── Conversions ────────────────────────────────────────────────

impl From<i64> for BigNum {
    fn from(v: i64) -> Self {
        BigNum::new(v)
    }
}

impl From<u64> for BigNum {
    fn from(v: u64) -> Self {
        BigNum::from_u64(v)
    }
}

impl From<i32> for BigNum {
    fn from(v: i32) -> Self {
        BigNum::new(v as i64)
    }
}

// ── Arithmetic operators ───────────────────────────────────────

impl Add for &BigNum {
    type Output = BigNum;
    fn add(self, rhs: Self) -> BigNum {
        BigNum(&self.0 + &rhs.0)
    }
}

impl Sub for &BigNum {
    type Output = BigNum;
    fn sub(self, rhs: Self) -> BigNum {
        BigNum(&self.0 - &rhs.0)
    }
}

impl Mul for &BigNum {
    type Output = BigNum;
    fn mul(self, rhs: Self) -> BigNum {
        BigNum(&self.0 * &rhs.0)
    }
}

impl Div for &BigNum {
    type Output = BigNum;
    fn div(self, rhs: Self) -> BigNum {
        BigNum(&self.0 / &rhs.0)
    }
}

impl Rem for &BigNum {
    type Output = BigNum;
    fn rem(self, rhs: Self) -> BigNum {
        BigNum(&self.0 % &rhs.0)
    }
}

impl Neg for &BigNum {
    type Output = BigNum;
    fn neg(self) -> BigNum {
        BigNum(-&self.0)
    }
}

// ── Owned variants (forward to ref impls) ──────────────────────

impl Add for BigNum {
    type Output = BigNum;
    fn add(self, rhs: Self) -> BigNum { &self + &rhs }
}

impl Sub for BigNum {
    type Output = BigNum;
    fn sub(self, rhs: Self) -> BigNum { &self - &rhs }
}

impl Mul for BigNum {
    type Output = BigNum;
    fn mul(self, rhs: Self) -> BigNum { &self * &rhs }
}

impl Div for BigNum {
    type Output = BigNum;
    fn div(self, rhs: Self) -> BigNum { &self / &rhs }
}

impl Rem for BigNum {
    type Output = BigNum;
    fn rem(self, rhs: Self) -> BigNum { &self % &rhs }
}

impl Neg for BigNum {
    type Output = BigNum;
    fn neg(self) -> BigNum { -&self }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_arithmetic() {
        let a = BigNum::new(10);
        let b = BigNum::new(3);
        assert_eq!((&a + &b).to_string(), "13");
        assert_eq!((&a - &b).to_string(), "7");
        assert_eq!((&a * &b).to_string(), "30");
        assert_eq!((&a / &b).to_string(), "3");
        assert_eq!((&a % &b).to_string(), "1");
        assert_eq!((-&b).to_string(), "-3");
    }

    #[test]
    fn from_str() {
        let n: BigNum = "12345678901234567890".parse().unwrap();
        assert_eq!(n.to_string(), "12345678901234567890");
    }
}

