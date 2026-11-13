//! WASM Bridge dispatcher for `rmathrs`.
//!
//! Provides the **single public WASM interface** exposed by `rmathrs` to JavaScript/TypeScript.
//! Unpacks requests, dispatches them to high-precision numerical functions in `rmathrs`,
//! and formats responses with zero precision loss.

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::config;
use crate::constants::{compute_e, compute_ln2, compute_pi, compute_pi_half, compute_two_pi};
use crate::functions::elementary;
use crate::numbers::{BigNum, Real};

/// Incoming bridge request payload.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct BridgeRequest {
    #[serde(default)]
    pub op: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub prec: Option<u64>,
}

/// Outgoing bridge response payload.
#[derive(Debug, Deserialize, Serialize)]
pub struct BridgeResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl BridgeResponse {
    pub fn ok(result: String) -> Self {
        Self {
            status: "ok".to_string(),
            result: Some(result),
            error: None,
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            status: "error".to_string(),
            result: None,
            error: Some(msg.into()),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|e| {
            format!("{{\"status\":\"error\",\"error\":\"Failed to serialize response: {}\"}}", e)
        })
    }
}

// ── Native Dispatch Logic ─────────────────────────────────────

/// Dispatches a operation with JSON payload string.
pub fn dispatch(op_param: &str, payload_json: &str) -> BridgeResponse {
    let req: BridgeRequest = if payload_json.trim().is_empty() {
        BridgeRequest::default()
    } else {
        match serde_json::from_str(payload_json) {
            Ok(r) => r,
            Err(e) => return BridgeResponse::err(format!("Invalid JSON payload: {}", e)),
        }
    };

    let op = if !op_param.is_empty() {
        op_param
    } else {
        &req.op
    };

    let prec = req.prec.unwrap_or_else(config::precision);

    let get_real_arg = |idx: usize| -> Result<Real, String> {
        let s = req.args.get(idx).ok_or_else(|| format!("Missing argument at index {}", idx))?;
        Real::from_str_with_prec(s, prec).map_err(|e| format!("Invalid real number '{}': {}", s, e))
    };

    let get_bignum_arg = |idx: usize| -> Result<BigNum, String> {
        let s = req.args.get(idx).ok_or_else(|| format!("Missing argument at index {}", idx))?;
        BigNum::from_str(s).map_err(|e| format!("Invalid integer '{}': {}", s, e))
    };

    let get_u64_arg = |idx: usize| -> Result<u64, String> {
        let s = req.args.get(idx).ok_or_else(|| format!("Missing argument at index {}", idx))?;
        s.parse::<u64>().map_err(|e| format!("Invalid u64 '{}': {}", s, e))
    };

    let get_i64_arg = |idx: usize| -> Result<i64, String> {
        let s = req.args.get(idx).ok_or_else(|| format!("Missing argument at index {}", idx))?;
        s.parse::<i64>().map_err(|e| format!("Invalid i64 '{}': {}", s, e))
    };

    match op {
        // ── Real Transcendental & Roots ───────────────────────
        "exp" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::exp(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },
        "ln" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::ln(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },
        "log" => match (get_real_arg(0), get_real_arg(1)) {
            (Ok(x), Ok(base)) => BridgeResponse::ok(elementary::log(&x, &base).to_string()),
            (Err(e), _) | (_, Err(e)) => BridgeResponse::err(e),
        },
        "sqrt" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::sqrt(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },
        "nroot" => match (get_real_arg(0), get_u64_arg(1)) {
            (Ok(x), Ok(n)) => BridgeResponse::ok(elementary::nroot(&x, n).to_string()),
            (Err(e), _) | (_, Err(e)) => BridgeResponse::err(e),
        },

        // ── Trigonometric ─────────────────────────────────────
        "sin" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::sin(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },
        "cos" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::cos(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },
        "tan" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::tan(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },

        // ── Inverse Trigonometric ─────────────────────────────
        "asin" | "arcsin" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::asin(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },
        "acos" | "arccos" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::acos(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },
        "atan" | "arctan" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::atan(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },

        // ── Hyperbolic ────────────────────────────────────────
        "sinh" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::sinh(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },
        "cosh" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::cosh(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },
        "tanh" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::tanh(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },

        // ── Inverse Hyperbolic ────────────────────────────────
        "asinh" | "arcsinh" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::asinh(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },
        "acosh" | "arccosh" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::acosh(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },
        "atanh" | "arctanh" => match get_real_arg(0) {
            Ok(x) => BridgeResponse::ok(elementary::atanh(&x).to_string()),
            Err(e) => BridgeResponse::err(e),
        },

        // ── Integer Arithmetic ────────────────────────────────
        "gcd" => match (get_bignum_arg(0), get_bignum_arg(1)) {
            (Ok(a), Ok(b)) => BridgeResponse::ok(elementary::gcd(&a, &b).to_string()),
            (Err(e), _) | (_, Err(e)) => BridgeResponse::err(e),
        },
        "lcm" => match (get_bignum_arg(0), get_bignum_arg(1)) {
            (Ok(a), Ok(b)) => BridgeResponse::ok(elementary::lcm(&a, &b).to_string()),
            (Err(e), _) | (_, Err(e)) => BridgeResponse::err(e),
        },
        "factorial" => match get_u64_arg(0) {
            Ok(n) => BridgeResponse::ok(elementary::factorial(n).to_string()),
            Err(e) => BridgeResponse::err(e),
        },
        "pow_int" => match (get_bignum_arg(0), get_u64_arg(1)) {
            (Ok(base), Ok(exp)) => BridgeResponse::ok(elementary::pow_int(&base, exp).to_string()),
            (Err(e), _) | (_, Err(e)) => BridgeResponse::err(e),
        },
        "pow_mod" => match (get_bignum_arg(0), get_bignum_arg(1), get_bignum_arg(2)) {
            (Ok(base), Ok(exp), Ok(m)) => {
                BridgeResponse::ok(elementary::pow_mod(&base, &exp, &m).to_string())
            }
            (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => BridgeResponse::err(e),
        },
        "pow_neg1" => match get_i64_arg(0) {
            Ok(n) => BridgeResponse::ok(elementary::pow_neg1(n).to_string()),
            Err(e) => BridgeResponse::err(e),
        },

        // ── Constants ─────────────────────────────────────────
        "pi" => BridgeResponse::ok(compute_pi(prec).to_string()),
        "pi_half" => BridgeResponse::ok(compute_pi_half(prec).to_string()),
        "two_pi" | "tau" => BridgeResponse::ok(compute_two_pi(prec).to_string()),
        "e" => BridgeResponse::ok(compute_e(prec).to_string()),
        "ln2" => BridgeResponse::ok(compute_ln2(prec).to_string()),

        other => BridgeResponse::err(format!("Unknown operation: '{}'", other)),
    }
}

// ── Single Public WASM Interface ──────────────────────────────

/// The **single public WebAssembly entrypoint** exposed to JavaScript.
///
/// Accepts an operation name and a JSON payload, executing the calculation
/// via Malachite and returning a serialized JSON response.
#[wasm_bindgen]
pub fn rmath_bridge_dispatch(op: &str, payload_json: &str) -> Result<String, JsValue> {
    Ok(dispatch(op, payload_json).to_json())
}

// ── Tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispatch_factorial() {
        let resp = dispatch("factorial", r#"{"args":["5"]}"#);
        assert_eq!(resp.status, "ok");
        assert_eq!(resp.result.unwrap(), "120");
    }

    #[test]
    fn test_dispatch_gcd() {
        let resp = dispatch("gcd", r#"{"args":["12","8"]}"#);
        assert_eq!(resp.status, "ok");
        assert_eq!(resp.result.unwrap(), "4");
    }

    #[test]
    fn test_dispatch_sin_zero() {
        let resp = dispatch("sin", r#"{"args":["0.0"]}"#);
        assert_eq!(resp.status, "ok");
        let val: f64 = resp.result.unwrap().parse().unwrap();
        assert!(val.abs() < 1e-12);
    }

    #[test]
    fn test_dispatch_exp_one() {
        let resp = dispatch("exp", r#"{"args":["1.0"]}"#);
        assert_eq!(resp.status, "ok");
        let val: f64 = resp.result.unwrap().parse().unwrap();
        assert!((val - std::f64::consts::E).abs() < 1e-12);
    }

    #[test]
    fn test_dispatch_unknown_op() {
        let resp = dispatch("non_existent_op", r#"{"args":[]}"#);
        assert_eq!(resp.status, "error");
        assert!(resp.error.unwrap().contains("Unknown operation"));
    }
}
