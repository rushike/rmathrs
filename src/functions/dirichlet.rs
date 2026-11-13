//! Dirichlet convolution — stub.
//!
//! Convolution over divisors. Requires factorize().

/// Builder for Dirichlet convolution `(f * g)(n) = Σ_{d|n} f(d) · g(n/d)`.
pub struct Dirichlet<F, G>
where
    F: Fn(u64) -> i64,
    G: Fn(u64) -> i64,
{
    pub f: F,
    pub g: G,
}

impl<F, G> Dirichlet<F, G>
where
    F: Fn(u64) -> i64,
    G: Fn(u64) -> i64,
{
    pub fn new(f: F, g: G) -> Self {
        Dirichlet { f, g }
    }

    /// Evaluate the Dirichlet convolution at `n`.
    pub fn eval(&self, _n: u64) -> i64 {
        todo!("dirichlet eval: requires divisor enumeration from factorize()")
    }
}

