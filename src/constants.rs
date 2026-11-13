//! Mathematical constants — computed directly using Malachite's arbitrary-precision constant functions.
//!
//! No hardcoded strings and no custom series rewriting:
//! - $\pi$: `malachite::Float::pi_prec(prec)`
//! - $\pi/2$: `malachite::Float::pi_over_2_prec(prec)`
//! - $2\pi$: `malachite::Float::tau_prec(prec)`
//! - $e$: `malachite::Float::e_prec(prec)`
//! - $\ln(2)$: `malachite::Float::ln_2_prec(prec)`

use std::sync::LazyLock;
use malachite::Float;
use crate::numbers::Real;

/// Default precision in bits used for caching global mathematical constants.
pub const CONSTANT_PRECISION_BITS: u64 = 512;

/// Dynamically compute $\pi$ using Malachite's arbitrary-precision algorithm.
pub fn compute_pi(prec: u64) -> Real {
    let (f, _) = Float::pi_prec(prec);
    Real::from_float(f, prec)
}

/// Dynamically compute $\pi / 2$ using Malachite's arbitrary-precision algorithm.
pub fn compute_pi_half(prec: u64) -> Real {
    let (f, _) = Float::pi_over_2_prec(prec);
    Real::from_float(f, prec)
}

/// Dynamically compute $2\pi$ ($\tau$) using Malachite's arbitrary-precision algorithm.
pub fn compute_two_pi(prec: u64) -> Real {
    let (f, _) = Float::tau_prec(prec);
    Real::from_float(f, prec)
}

/// Dynamically compute Euler's number $e$ using Malachite's arbitrary-precision algorithm.
pub fn compute_e(prec: u64) -> Real {
    let (f, _) = Float::e_prec(prec);
    Real::from_float(f, prec)
}

/// Dynamically compute $\ln(2)$ using Malachite's arbitrary-precision algorithm.
pub fn compute_ln2(prec: u64) -> Real {
    let (f, _) = Float::ln_2_prec(prec);
    Real::from_float(f, prec)
}

// ── Lazy Static Constants ─────────────────────────────────────

/// $\pi$ — computed once at 512 bits via Malachite, cached forever.
pub static PI: LazyLock<Real> = LazyLock::new(|| compute_pi(CONSTANT_PRECISION_BITS));

/// $\pi/2$ — computed once at 512 bits via Malachite, cached forever.
pub static PI_HALF: LazyLock<Real> = LazyLock::new(|| compute_pi_half(CONSTANT_PRECISION_BITS));

/// $2\pi$ — computed once at 512 bits via Malachite, cached forever.
pub static TWO_PI: LazyLock<Real> = LazyLock::new(|| compute_two_pi(CONSTANT_PRECISION_BITS));

/// $e$ — computed once at 512 bits via Malachite, cached forever.
pub static E: LazyLock<Real> = LazyLock::new(|| compute_e(CONSTANT_PRECISION_BITS));

/// $\ln(2)$ — computed once at 512 bits via Malachite, cached forever.
pub static LN2: LazyLock<Real> = LazyLock::new(|| compute_ln2(CONSTANT_PRECISION_BITS));

// ── Tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-14;

    #[test]
    fn test_computed_pi() {
        let pi = compute_pi(128);
        assert!((pi.to_f64() - std::f64::consts::PI).abs() < TOL);
        assert!((PI.to_f64() - std::f64::consts::PI).abs() < TOL);
    }

    #[test]
    fn test_computed_pi_half() {
        assert!((PI_HALF.to_f64() - std::f64::consts::FRAC_PI_2).abs() < TOL);
    }

    #[test]
    fn test_computed_two_pi() {
        assert!((TWO_PI.to_f64() - 2.0 * std::f64::consts::PI).abs() < TOL);
    }

    #[test]
    fn test_computed_e() {
        let e = compute_e(128);
        assert!((e.to_f64() - std::f64::consts::E).abs() < TOL);
        assert!((E.to_f64() - std::f64::consts::E).abs() < TOL);
    }

    #[test]
    fn test_computed_ln2() {
        let ln2 = compute_ln2(128);
        assert!((ln2.to_f64() - std::f64::consts::LN_2).abs() < TOL);
        assert!((LN2.to_f64() - std::f64::consts::LN_2).abs() < TOL);
    }
}
