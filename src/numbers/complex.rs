//! Complex number — `a + bi`, backed by two [`Real`] values.
//!
//! Malachite has no Complex type, so arithmetic is implemented manually.

use super::real::Real;
use std::fmt;

/// Complex number `a + bi`.
#[derive(Clone, Debug)]
pub struct Complex {
    /// Real part.
    pub re: Real,
    /// Imaginary part.
    pub im: Real,
}

impl Complex {
    pub fn new(re: Real, im: Real) -> Self {
        Complex { re, im }
    }

    pub fn from_real(r: Real) -> Self {
        Complex {
            re: r,
            im: Real::zero(),
        }
    }

    pub fn zero() -> Self {
        Complex::new(Real::zero(), Real::zero())
    }

    pub fn one() -> Self {
        Complex::new(Real::one(), Real::zero())
    }

    // ── Arithmetic ───────────────────────────────────────────

    /// `(a+bi) + (c+di) = (a+c) + (b+d)i`
    pub fn add(&self, other: &Self) -> Self {
        Complex::new(self.re.add(&other.re), self.im.add(&other.im))
    }

    /// `(a+bi) - (c+di) = (a-c) + (b-d)i`
    pub fn sub(&self, other: &Self) -> Self {
        Complex::new(self.re.sub(&other.re), self.im.sub(&other.im))
    }

    /// `(a+bi)(c+di) = (ac-bd) + (ad+bc)i`
    pub fn mul(&self, other: &Self) -> Self {
        let re = self.re.mul(&other.re).sub(&self.im.mul(&other.im));
        let im = self.re.mul(&other.im).add(&self.im.mul(&other.re));
        Complex::new(re, im)
    }

    /// `(a+bi)/(c+di) = [(a+bi)(c-di)] / (c²+d²)`
    pub fn div(&self, other: &Self) -> Self {
        let conj = other.conjugate();
        let num = self.mul(&conj);
        let den = other.re.square().add(&other.im.square());
        Complex::new(num.re.div(&den), num.im.div(&den))
    }

    /// `conj(a+bi) = a - bi`
    pub fn conjugate(&self) -> Self {
        Complex::new(self.re.clone(), self.im.neg())
    }

    /// `|a+bi| = sqrt(a² + b²)`
    pub fn abs(&self) -> Real {
        let sum = self.re.square().add(&self.im.square());
        crate::functions::elementary::sqrt(&sum)
    }

    pub fn square(&self) -> Self {
        self.mul(self)
    }

    pub fn real(&self) -> &Real {
        &self.re
    }

    pub fn imaginary(&self) -> &Real {
        &self.im
    }

    pub fn neg(&self) -> Self {
        Complex::new(self.re.neg(), self.im.neg())
    }
}

// ── Display ────────────────────────────────────────────────────

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+{}i", self.re, self.im)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_add() {
        let a = Complex::new(Real::new(1.0), Real::new(2.0));
        let b = Complex::new(Real::new(3.0), Real::new(4.0));
        let c = a.add(&b);
        assert!((c.re.to_f64() - 4.0).abs() < 1e-10);
        assert!((c.im.to_f64() - 6.0).abs() < 1e-10);
    }

    #[test]
    fn complex_mul() {
        // (1+2i)(3+4i) = (3-8) + (4+6)i = -5 + 10i
        let a = Complex::new(Real::new(1.0), Real::new(2.0));
        let b = Complex::new(Real::new(3.0), Real::new(4.0));
        let c = a.mul(&b);
        assert!((c.re.to_f64() - (-5.0)).abs() < 1e-10);
        assert!((c.im.to_f64() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn conjugate() {
        let a = Complex::new(Real::new(3.0), Real::new(4.0));
        let c = a.conjugate();
        assert!((c.re.to_f64() - 3.0).abs() < 1e-10);
        assert!((c.im.to_f64() - (-4.0)).abs() < 1e-10);
    }
}

