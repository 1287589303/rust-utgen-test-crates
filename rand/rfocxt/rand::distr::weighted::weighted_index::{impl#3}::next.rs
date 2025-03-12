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
impl<X> Iterator for WeightedIndexIter<'_, X>
where
    X: for<'b> core::ops::SubAssign<&'b X> + SampleUniform + PartialOrd + Clone,
{
    type Item = X;
    fn next(&mut self) -> Option<Self::Item> {
        match self.weighted_index.weight(self.index) {
            None => None,
            Some(weight) => {
                self.index += 1;
                Some(weight)
            }
        }
    }
}
impl<X: SampleUniform + PartialOrd + Clone> WeightedIndex<X> {
    pub fn weight(&self, index: usize) -> Option<X>
    where
        X: for<'a> core::ops::SubAssign<&'a X>,
    {
        use core::cmp::Ordering::*;
        let mut weight = match index.cmp(&self.cumulative_weights.len()) {
            Less => self.cumulative_weights[index].clone(),
            Equal => self.total_weight.clone(),
            Greater => return None,
        };
        if index > 0 {
            weight -= &self.cumulative_weights[index - 1];
        }
        Some(weight)
    }
    pub fn weights(&self) -> WeightedIndexIter<'_, X>
    where
        X: for<'a> core::ops::SubAssign<&'a X>,
    {}
    pub fn total_weight(&self) -> X {}
}
