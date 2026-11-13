//! rmathrs — Arbitrary precision arithmetic and math library.
//!
//! Rust port of `rmathjs`, backed by malachite for arbitrary-precision types.

pub mod config;
pub mod error;

#[cfg(feature = "malachite")]
pub mod bridge;

#[cfg(feature = "malachite")]
pub mod constants;

#[cfg(feature = "malachite")]
pub mod functions;

#[cfg(feature = "malachite")]
pub mod numbers;

#[cfg(feature = "malachite")]
pub use numbers::{BigNum, Complex, Fraction, Real};

#[cfg(feature = "malachite")]
pub use bridge::rmath_bridge_dispatch;
