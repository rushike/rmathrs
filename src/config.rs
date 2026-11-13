//! Global configuration for rmathrs — precision and defaults.

use std::sync::{LazyLock, RwLock};

/// Default precision in bits.
///   100 bits ≈ 30 decimal digits
///   166 bits ≈ 50 decimal digits
/// Change this constant to adjust the library-wide default.
const DEFAULT_PRECISION_BITS: u64 = 100;

/// Global math configuration.
pub struct RMathConfig {
    /// Float precision in bits.
    pub precision: u64,
}

impl Default for RMathConfig {
    fn default() -> Self {
        RMathConfig {
            precision: DEFAULT_PRECISION_BITS,
        }
    }
}

static CONFIG: LazyLock<RwLock<RMathConfig>> =
    LazyLock::new(|| RwLock::new(RMathConfig::default()));

/// Get the current default precision in bits.
pub fn precision() -> u64 {
    CONFIG.read().unwrap().precision
}

/// Set the global default precision in bits.
///
/// Reference:
///   100 bits ≈ 30 decimal digits
///   166 bits ≈ 50 decimal digits
///   332 bits ≈ 100 decimal digits
pub fn set_precision(bits: u64) {
    CONFIG.write().unwrap().precision = bits;
}

