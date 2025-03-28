use crate::distr::uniform::{SampleRange, SampleUniform};
use crate::distr::{self, Distribution, StandardUniform};
use core::num::Wrapping;
use rand_core::RngCore;
use zerocopy::IntoBytes;
pub trait Rng: RngCore {
    #[inline]
    fn random<T>(&mut self) -> T
    where
        StandardUniform: Distribution<T>;
    #[inline]
    fn random_iter<T>(self) -> distr::Iter<StandardUniform, Self, T>
    where
        Self: Sized,
        StandardUniform: Distribution<T>,
    {
        StandardUniform.sample_iter(self)
    }
    #[track_caller]
    fn random_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>;
    #[inline]
    #[track_caller]
    fn random_bool(&mut self, p: f64) -> bool;
    #[inline]
    #[track_caller]
    fn random_ratio(&mut self, numerator: u32, denominator: u32) -> bool {
        match distr::Bernoulli::from_ratio(numerator, denominator) {
            Ok(d) => self.sample(d),
            Err(_) => {
                panic!("p={}/{} is outside range [0.0, 1.0]", numerator, denominator)
            }
        }
    }
    fn sample<T, D: Distribution<T>>(&mut self, distr: D) -> T;
    fn sample_iter<T, D>(self, distr: D) -> distr::Iter<D, Self, T>
    where
        D: Distribution<T>,
        Self: Sized,
    {
        distr.sample_iter(self)
    }
    #[track_caller]
    fn fill<T: Fill + ?Sized>(&mut self, dest: &mut T);
    #[inline]
    #[deprecated(
        since = "0.9.0",
        note = "Renamed to `random` to avoid conflict with the new `gen` keyword in Rust 2024."
    )]
    fn r#gen<T>(&mut self) -> T
    where
        StandardUniform: Distribution<T>;
    #[inline]
    #[deprecated(since = "0.9.0", note = "Renamed to `random_range`")]
    fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>;
    #[inline]
    #[deprecated(since = "0.9.0", note = "Renamed to `random_bool`")]
    fn gen_bool(&mut self, p: f64) -> bool;
    #[inline]
    #[deprecated(since = "0.9.0", note = "Renamed to `random_ratio`")]
    fn gen_ratio(&mut self, numerator: u32, denominator: u32) -> bool;
}
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bernoulli {
    /// Probability of success, relative to the maximal integer.
    p_int: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BernoulliError {
    /// `p < 0` or `p > 1`.
    InvalidProbability,
}
impl Bernoulli {
    #[inline]
    pub fn new(p: f64) -> Result<Bernoulli, BernoulliError> {}
    #[inline]
    pub fn from_ratio(
        numerator: u32,
        denominator: u32,
    ) -> Result<Bernoulli, BernoulliError> {
        if numerator > denominator || denominator == 0 {
            return Err(BernoulliError::InvalidProbability);
        }
        if numerator == denominator {
            return Ok(Bernoulli { p_int: ALWAYS_TRUE });
        }
        let p_int = ((f64::from(numerator) / f64::from(denominator)) * SCALE) as u64;
        Ok(Bernoulli { p_int })
    }
    #[inline]
    pub fn p(&self) -> f64 {}
}
