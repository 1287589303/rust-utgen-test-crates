pub use float::UniformFloat;
pub use int::{UniformInt, UniformUsize};
pub use other::{UniformChar, UniformDuration};
use core::fmt;
use core::ops::{Range, RangeInclusive, RangeTo, RangeToInclusive};
use crate::distr::Distribution;
use crate::{Rng, RngCore};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
pub trait SampleRange<T> {
    fn sample_single<R: RngCore + ?Sized>(self, rng: &mut R) -> Result<T, Error>;
    fn is_empty(&self) -> bool;
}
pub trait SampleUniform: Sized {
    type Sampler: UniformSampler<X = Self>;
}
impl<T: SampleUniform + PartialOrd> SampleRange<T> for RangeInclusive<T> {
    #[inline]
    fn sample_single<R: RngCore + ?Sized>(self, rng: &mut R) -> Result<T, Error> {}
    #[inline]
    fn is_empty(&self) -> bool {
        !(self.start() <= self.end())
    }
}
