use super::{Error, Weight};
use crate::distr::uniform::{SampleBorrow, SampleUniform, UniformSampler};
use crate::distr::Distribution;
use crate::Rng;
use alloc::vec::Vec;
use core::fmt::{self, Debug};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
pub trait SampleUniform: Sized {
    type Sampler: UniformSampler<X = Self>;
}
pub struct WeightedIndexIter<'a, X: SampleUniform + PartialOrd> {
    weighted_index: &'a WeightedIndex<X>,
    index: usize,
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WeightedIndex<X: SampleUniform + PartialOrd> {
    cumulative_weights: Vec<X>,
    total_weight: X,
    weight_distribution: X::Sampler,
}
impl<X> Clone for WeightedIndexIter<'_, X>
where
    X: SampleUniform + PartialOrd,
{
    fn clone(&self) -> Self {
        WeightedIndexIter {
            weighted_index: self.weighted_index,
            index: self.index,
        }
    }
}
