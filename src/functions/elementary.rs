//! Elementary mathematical functions.
//!
//! Ported from `rmathjs/src/functions/elementary.ts`.
//! Integer functions delegate directly to malachite arithmetic traits.
//! Real transcendental functions use arbitrary-precision series / iterations
//! over [`Real`].

use malachite::base::num::arithmetic::traits::{Factorial, Gcd, Lcm, ModPow, Pow};
use malachite::{Float, Integer, Natural};

use crate::numbers::{BigNum, Real};

// ── Integer Functions ─────────────────────────────────────────

/// Greatest common divisor.
pub fn gcd(a: &BigNum, b: &BigNum) -> BigNum {
    let a_nat = a.inner().unsigned_abs_ref();
    let b_nat = b.inner().unsigned_abs_ref();
    BigNum::from_integer(Integer::from(a_nat.gcd(b_nat)))
}

/// Least common multiple.
pub fn lcm(a: &BigNum, b: &BigNum) -> BigNum {
    let a_nat = a.inner().unsigned_abs_ref();
    let b_nat = b.inner().unsigned_abs_ref();
    BigNum::from_integer(Integer::from(a_nat.lcm(b_nat)))
}

/// Factorial: `n!`
pub fn factorial(n: u64) -> BigNum {
    BigNum::from_integer(Integer::from(Natural::factorial(n)))
}

/// Integer power: `a ^ n`
pub fn pow_int(a: &BigNum, n: u64) -> BigNum {
    BigNum::from_integer(a.inner().pow(n))
}

/// Modular exponentiation: `a^n mod m`
pub fn pow_mod(a: &BigNum, n: &BigNum, m: &BigNum) -> BigNum {
    let base_nat = a.inner().unsigned_abs_ref();
    let exp_nat = n.inner().unsigned_abs_ref();
    let m_nat = m.inner().unsigned_abs_ref();
    let res_nat = base_nat.mod_pow(exp_nat, m_nat);
    BigNum::from_integer(Integer::from(res_nat))
}

/// `(-1)^n` — returns -1 for odd n, 1 for even n.
pub fn pow_neg1(n: i64) -> i64 {
    if n & 1 == 1 {
        -1
    } else {
        1
    }
}

// ── Square root & nth root ────────────────────────────────────

/// Arbitrary-precision square root delegating directly to Malachite.
pub fn sqrt(x: &Real) -> Real {
    let (f, _) = Float::sqrt_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

/// Arbitrary-precision nth root delegating directly to Malachite.
pub fn nroot(x: &Real, n: u64) -> Real {
    let (f, _) = Float::root_u_prec_ref(x.inner(), n, x.prec());
    Real::from_float(f, x.prec())
}

// ── Exponential & Logarithm ───────────────────────────────────

/// Arbitrary-precision exponential $e^x$ delegating directly to Malachite.
pub fn exp(x: &Real) -> Real {
    let (f, _) = Float::exp_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

/// Arbitrary-precision natural logarithm $\ln(x)$ delegating directly to Malachite.
pub fn ln(x: &Real) -> Real {
    let (f, _) = Float::ln_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

/// Logarithm base `b`: $\log_b(x) = \ln(x) / \ln(b)$.
pub fn log(x: &Real, base: &Real) -> Real {
    ln(x).div(&ln(base))
}

// ── Trigonometric ─────────────────────────────────────────────

/// Arbitrary-precision sine delegating directly to Malachite.
pub fn sin(x: &Real) -> Real {
    let (f, _) = Float::sin_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

/// Arbitrary-precision cosine delegating directly to Malachite.
pub fn cos(x: &Real) -> Real {
    let (f, _) = Float::cos_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

/// Arbitrary-precision tangent delegating directly to Malachite.
pub fn tan(x: &Real) -> Real {
    let (f, _) = Float::tan_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

// ── Inverse Trigonometric ─────────────────────────────────────

/// Arbitrary-precision arc sine delegating directly to Malachite.
pub fn asin(x: &Real) -> Real {
    let (f, _) = Float::asin_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

/// Arbitrary-precision arc cosine delegating directly to Malachite.
pub fn acos(x: &Real) -> Real {
    let (f, _) = Float::acos_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

/// Arbitrary-precision arc tangent delegating directly to Malachite.
pub fn atan(x: &Real) -> Real {
    let (f, _) = Float::atan_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

// ── Hyperbolic ────────────────────────────────────────────────

/// Arbitrary-precision hyperbolic sine delegating directly to Malachite.
pub fn sinh(x: &Real) -> Real {
    let (f, _) = Float::sinh_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

/// Arbitrary-precision hyperbolic cosine delegating directly to Malachite.
pub fn cosh(x: &Real) -> Real {
    let (f, _) = Float::cosh_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

/// Arbitrary-precision hyperbolic tangent delegating directly to Malachite.
pub fn tanh(x: &Real) -> Real {
    let (f, _) = Float::tanh_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

// ── Inverse Hyperbolic ────────────────────────────────────────

/// Arbitrary-precision inverse hyperbolic sine delegating directly to Malachite.
pub fn asinh(x: &Real) -> Real {
    let (f, _) = Float::asinh_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

/// Arbitrary-precision inverse hyperbolic cosine delegating directly to Malachite.
pub fn acosh(x: &Real) -> Real {
    let (f, _) = Float::acosh_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

/// Arbitrary-precision inverse hyperbolic tangent delegating directly to Malachite.
pub fn atanh(x: &Real) -> Real {
    let (f, _) = Float::atanh_prec_ref(x.inner(), x.prec());
    Real::from_float(f, x.prec())
}

// ── Tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::PI_HALF;

    const TOL: f64 = 1e-12;

    // ── Integer ───────────────────────────────────────────────

    #[test]
    fn test_gcd() {
        let a = BigNum::new(12);
        let b = BigNum::new(8);
        assert_eq!(gcd(&a, &b).to_string(), "4");
    }

    #[test]
    fn test_lcm() {
        let a = BigNum::new(4);
        let b = BigNum::new(6);
        assert_eq!(lcm(&a, &b).to_string(), "12");
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0).to_string(), "1");
        assert_eq!(factorial(1).to_string(), "1");
        assert_eq!(factorial(10).to_string(), "3628800");
        assert_eq!(
            factorial(20).to_string(),
            "2432902008176640000"
        );
    }

    #[test]
    fn test_pow_int() {
        let a = BigNum::new(2);
        assert_eq!(pow_int(&a, 10).to_string(), "1024");
    }

    #[test]
    fn test_pow_mod() {
        let a = BigNum::new(2);
        let n = BigNum::new(10);
        let m = BigNum::new(1000);
        assert_eq!(pow_mod(&a, &n, &m).to_string(), "24");
    }

    #[test]
    fn test_pow_neg1() {
        assert_eq!(pow_neg1(0), 1);
        assert_eq!(pow_neg1(1), -1);
        assert_eq!(pow_neg1(2), 1);
        assert_eq!(pow_neg1(3), -1);
    }

    // ── Exp / Ln ──────────────────────────────────────────────

    #[test]
    fn test_exp_1() {
        let one = Real::new(1.0);
        let e = exp(&one);
        assert!((e.to_f64() - std::f64::consts::E).abs() < TOL);
    }

    #[test]
    fn test_ln_e() {
        let e_val = Real::new(std::f64::consts::E);
        let result = ln(&e_val);
        assert!((result.to_f64() - 1.0).abs() < TOL);
    }

    #[test]
    fn test_log_base() {
        let x = Real::new(1000.0);
        let base = Real::new(10.0);
        let result = log(&x, &base);
        assert!((result.to_f64() - 3.0).abs() < 1e-8);
    }

    // ── Trig ──────────────────────────────────────────────────

    #[test]
    fn test_sin_zero() {
        let z = Real::new(0.0);
        assert!((sin(&z).to_f64()).abs() < TOL);
    }

    #[test]
    fn test_cos_zero() {
        let z = Real::new(0.0);
        assert!((cos(&z).to_f64() - 1.0).abs() < TOL);
    }

    #[test]
    fn test_sin_pi_half() {
        let pi_2 = PI_HALF.clone();
        assert!((sin(&pi_2).to_f64() - 1.0).abs() < TOL);
    }

    #[test]
    fn test_tan() {
        let pi_4 = PI_HALF.mul(&Real::new(0.5));
        assert!((tan(&pi_4).to_f64() - 1.0).abs() < 1e-8);
    }

    // ── Inverse Trig ──────────────────────────────────────────

    #[test]
    fn test_asin_1() {
        let one = Real::new(1.0);
        let result = asin(&one);
        assert!(
            (result.to_f64() - std::f64::consts::FRAC_PI_2).abs() < 1e-3
        );
    }

    #[test]
    fn test_atan_1() {
        let one = Real::new(1.0);
        let result = atan(&one);
        assert!(
            (result.to_f64() - std::f64::consts::FRAC_PI_4).abs() < TOL
        );
    }

    // ── Hyperbolic ────────────────────────────────────────────

    #[test]
    fn test_sinh_zero() {
        let z = Real::new(0.0);
        assert!((sinh(&z).to_f64()).abs() < TOL);
    }

    #[test]
    fn test_cosh_zero() {
        let z = Real::new(0.0);
        assert!((cosh(&z).to_f64() - 1.0).abs() < TOL);
    }

    #[test]
    fn test_tanh_zero() {
        let z = Real::new(0.0);
        assert!((tanh(&z).to_f64()).abs() < TOL);
    }

    // ── Inverse Hyperbolic ────────────────────────────────────

    #[test]
    fn test_asinh_zero() {
        let z = Real::new(0.0);
        assert!((asinh(&z).to_f64()).abs() < TOL);
    }

    // ── Sqrt / Nroot ──────────────────────────────────────────

    #[test]
    fn test_sqrt() {
        let x = Real::new(4.0);
        assert!((sqrt(&x).to_f64() - 2.0).abs() < TOL);
    }

    #[test]
    fn test_nroot_cube() {
        let x = Real::new(27.0);
        let result = nroot(&x, 3);
        assert!((result.to_f64() - 3.0).abs() < 1e-8);
    }
}
