//! Arbitrary-precision real number — lean wrapper over `malachite::Float`.
//!
//! This struct provides **only** basic arithmetic, comparison, and conversion.
//! Transcendental functions (sin, cos, exp, ln, …) live in
//! [`crate::functions::elementary`].

use malachite::base::num::arithmetic::traits::Abs;
use malachite::base::num::basic::traits::{
    NegativeOne, One, OneHalf, Two, Zero,
};
use malachite::base::num::conversion::traits::{FromSciString, IsInteger, RoundingFrom};
use malachite::base::rounding_modes::RoundingMode;
use malachite::{Float, Integer, Rational};
use std::fmt;
use std::str::FromStr;

use crate::config;

/// Arbitrary-precision real number.
///
/// Intentionally lean — transcendental functions are free functions in
/// `functions::elementary`, not methods on this struct.
#[derive(Clone, Debug)]
pub struct Real {
    pub(crate) inner: Float,
    pub(crate) prec: u64,
}

impl Real {
    // ── Construction ──────────────────────────────────────────

    pub fn new(val: f64) -> Self {
        Real {
            inner: Float::from(val),
            prec: config::precision(),
        }
    }

    pub fn from_float(f: Float, prec: u64) -> Self {
        Real { inner: f, prec }
    }

    pub fn from_i64(v: i64) -> Self {
        Real {
            inner: Float::from(v),
            prec: config::precision(),
        }
    }

    pub fn from_u64(v: u64) -> Self {
        Real {
            inner: Float::from(v),
            prec: config::precision(),
        }
    }

    pub fn from_str_with_prec(s: &str, prec: u64) -> Result<Self, crate::error::RMathError> {
        let q = Rational::from_sci_string(s)
            .ok_or_else(|| crate::error::RMathError::InvalidNumberFormat(s.to_string()))?;
        let (f, _) = Float::from_rational_prec(q, prec);
        Ok(Real { inner: f, prec })
    }

    pub fn with_precision(mut self, prec: u64) -> Self {
        self.prec = prec;
        self
    }

    pub fn zero() -> Self {
        Real {
            inner: Float::ZERO,
            prec: config::precision(),
        }
    }

    pub fn one() -> Self {
        Real {
            inner: Float::ONE,
            prec: config::precision(),
        }
    }

    pub fn two() -> Self {
        Real {
            inner: Float::TWO,
            prec: config::precision(),
        }
    }

    pub fn negative_one() -> Self {
        Real {
            inner: Float::NEGATIVE_ONE,
            prec: config::precision(),
        }
    }

    pub fn one_half() -> Self {
        Real {
            inner: Float::ONE_HALF,
            prec: config::precision(),
        }
    }

    // ── Accessors ────────────────────────────────────────────

    pub fn inner(&self) -> &Float {
        &self.inner
    }

    pub fn into_inner(self) -> Float {
        self.inner
    }

    pub fn prec(&self) -> u64 {
        self.prec
    }

    // ── Basic Arithmetic ─────────────────────────────────────

    pub fn add(&self, other: &Self) -> Self {
        let p = self.prec.max(other.prec);
        let (r, _) = (&self.inner).add_prec_ref_ref(&other.inner, p);
        Real { inner: r, prec: p }
    }

    pub fn sub(&self, other: &Self) -> Self {
        let p = self.prec.max(other.prec);
        let (r, _) = (&self.inner).sub_prec_ref_ref(&other.inner, p);
        Real { inner: r, prec: p }
    }

    pub fn mul(&self, other: &Self) -> Self {
        let p = self.prec.max(other.prec);
        let (r, _) = (&self.inner).mul_prec_ref_ref(&other.inner, p);
        Real { inner: r, prec: p }
    }

    pub fn div(&self, other: &Self) -> Self {
        let p = self.prec.max(other.prec);
        let (r, _) = (&self.inner).div_prec_ref_ref(&other.inner, p);
        Real { inner: r, prec: p }
    }

    pub fn neg(&self) -> Self {
        Real {
            inner: -&self.inner,
            prec: self.prec,
        }
    }

    pub fn abs(&self) -> Self {
        Real {
            inner: (&self.inner).abs(),
            prec: self.prec,
        }
    }

    pub fn square(&self) -> Self {
        let (r, _) = (&self.inner).square_prec_ref(self.prec);
        Real { inner: r, prec: self.prec }
    }

    pub fn recip(&self) -> Self {
        let (r, _) = (&self.inner).reciprocal_prec_ref(self.prec);
        Real { inner: r, prec: self.prec }
    }

    pub fn rem(&self, other: &Self) -> Self {
        let q = self.div(other).floor();
        self.sub(&q.mul(other))
    }

    // ── Comparison ───────────────────────────────────────────

    pub fn lt(&self, other: &Self) -> bool {
        self.inner < other.inner
    }

    pub fn le(&self, other: &Self) -> bool {
        self.inner <= other.inner
    }

    pub fn gt(&self, other: &Self) -> bool {
        self.inner > other.inner
    }

    pub fn ge(&self, other: &Self) -> bool {
        self.inner >= other.inner
    }

    // ── Conversion ───────────────────────────────────────────

    /// Convert to f64 (lossy for high-precision values).
    pub fn to_f64(&self) -> f64 {
        f64::rounding_from(&self.inner, RoundingMode::Nearest).0
    }

    /// Convert to BigNum (floor).
    pub fn to_bignum(&self) -> super::integer::BigNum {
        let (i, _) = Integer::rounding_from(&self.inner, RoundingMode::Floor);
        super::integer::BigNum::from_integer(i)
    }

    pub fn floor(&self) -> Self {
        let (i, _) = Integer::rounding_from(&self.inner, RoundingMode::Floor);
        let (f, _) = Float::from_integer_prec_round_ref(&i, self.prec, RoundingMode::Nearest);
        Real::from_float(f, self.prec)
    }

    pub fn ceil(&self) -> Self {
        let (i, _) = Integer::rounding_from(&self.inner, RoundingMode::Ceiling);
        let (f, _) = Float::from_integer_prec_round_ref(&i, self.prec, RoundingMode::Nearest);
        Real::from_float(f, self.prec)
    }

    pub fn is_integer(&self) -> bool {
        (&self.inner).is_integer()
    }
}

// ── Display ────────────────────────────────────────────────────

impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl FromStr for Real {
    type Err = crate::error::RMathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Real::from_str_with_prec(s, config::precision())
    }
}

// ── From impls ─────────────────────────────────────────────────

impl From<f64> for Real {
    fn from(v: f64) -> Self {
        Real::new(v)
    }
}

impl From<i64> for Real {
    fn from(v: i64) -> Self {
        Real::from_i64(v)
    }
}

impl From<u64> for Real {
    fn from(v: u64) -> Self {
        Real::from_u64(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_arithmetic() {
        let a = Real::new(3.0);
        let b = Real::new(2.0);
        let sum = a.add(&b);
        assert!((sum.to_f64() - 5.0).abs() < 1e-10);

        let prod = a.mul(&b);
        assert!((prod.to_f64() - 6.0).abs() < 1e-10);

        let diff = a.sub(&b);
        assert!((diff.to_f64() - 1.0).abs() < 1e-10);

        let quot = a.div(&b);
        assert!((quot.to_f64() - 1.5).abs() < 1e-10);
    }

    #[test]
    fn negation_and_abs() {
        let a = Real::new(5.0);
        let neg = a.neg();
        assert!((neg.to_f64() + 5.0).abs() < 1e-10);
        assert!((neg.abs().to_f64() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn parse_decimal_string() {
        let r: Real = "123.456789".parse().unwrap();
        assert!((r.to_f64() - 123.456789).abs() < 1e-6);
    }
}
