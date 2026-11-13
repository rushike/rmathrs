//! Arbitrary-precision fraction — thin wrapper over `malachite::Rational`.
//!
//! Auto-reduces via GCD on construction (handled by malachite).

use malachite::Rational;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::str::FromStr;

/// Arbitrary-precision fraction (auto-reduces via GCD).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fraction(pub Rational);

impl Fraction {
    pub fn new(num: i64, den: i64) -> Self {
        Fraction(Rational::from_signeds(num, den))
    }

    pub fn from_rational(r: Rational) -> Self {
        Fraction(r)
    }

    pub fn inner(&self) -> &Rational {
        &self.0
    }

    pub fn into_inner(self) -> Rational {
        self.0
    }

    pub fn add(&self, other: &Self) -> Self {
        Fraction(&self.0 + &other.0)
    }

    pub fn sub(&self, other: &Self) -> Self {
        Fraction(&self.0 - &other.0)
    }

    pub fn mul(&self, other: &Self) -> Self {
        Fraction(&self.0 * &other.0)
    }

    pub fn div(&self, other: &Self) -> Self {
        Fraction(&self.0 / &other.0)
    }

    pub fn neg(&self) -> Self {
        Fraction(-&self.0)
    }
}

// ── Display / FromStr ──────────────────────────────────────────

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Fraction {
    type Err = crate::error::RMathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Accept "n/d" format
        if let Some((num_str, den_str)) = s.split_once('/') {
            let num = num_str
                .trim()
                .parse::<i64>()
                .map_err(|_| crate::error::RMathError::InvalidNumberFormat(s.to_string()))?;
            let den = den_str
                .trim()
                .parse::<i64>()
                .map_err(|_| crate::error::RMathError::InvalidNumberFormat(s.to_string()))?;
            Ok(Fraction::new(num, den))
        } else {
            Err(crate::error::RMathError::InvalidNumberFormat(
                format!("Expected 'n/d' format, got: {}", s),
            ))
        }
    }
}

// ── From impls ─────────────────────────────────────────────────

impl From<(i64, i64)> for Fraction {
    fn from((n, d): (i64, i64)) -> Self {
        Fraction::new(n, d)
    }
}

// ── Operator impls (ref) ───────────────────────────────────────

impl Add for &Fraction {
    type Output = Fraction;
    fn add(self, rhs: Self) -> Fraction { self.add(rhs) }
}

impl Sub for &Fraction {
    type Output = Fraction;
    fn sub(self, rhs: Self) -> Fraction { Fraction::sub(self, rhs) }
}

impl Mul for &Fraction {
    type Output = Fraction;
    fn mul(self, rhs: Self) -> Fraction { Fraction::mul(self, rhs) }
}

impl Div for &Fraction {
    type Output = Fraction;
    fn div(self, rhs: Self) -> Fraction { Fraction::div(self, rhs) }
}

impl Neg for &Fraction {
    type Output = Fraction;
    fn neg(self) -> Fraction { Fraction::neg(self) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_reduces() {
        let f = Fraction::new(4, 6);
        assert_eq!(f.to_string(), "2/3");
    }

    #[test]
    fn arithmetic() {
        let a = Fraction::new(1, 3);
        let b = Fraction::new(1, 6);
        let sum = a.add(&b);
        assert_eq!(sum.to_string(), "1/2");

        let prod = a.mul(&b);
        assert_eq!(prod.to_string(), "1/18");
    }

    #[test]
    fn from_str_roundtrip() {
        let f: Fraction = "6/8".parse().unwrap();
        assert_eq!(f.to_string(), "3/4");
    }
}

