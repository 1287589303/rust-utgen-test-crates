use super::increasing_uniform::IncreasingUniform;
use super::index;
#[cfg(feature = "alloc")]
use crate::distr::uniform::{SampleBorrow, SampleUniform};
#[cfg(feature = "alloc")]
use crate::distr::weighted::{Error as WeightError, Weight};
use crate::Rng;
use core::ops::{Index, IndexMut};
#[cfg(feature = "alloc")]
#[derive(Debug)]
pub struct SliceChooseIter<'a, S: ?Sized + 'a, T: 'a> {
    slice: &'a S,
    _phantom: core::marker::PhantomData<T>,
    indices: index::IndexVecIntoIter,
}
#[derive(Clone, Debug)]
pub enum IndexVecIntoIter {
    #[doc(hidden)]
    U32(vec::IntoIter<u32>),
    #[cfg(target_pointer_width = "64")]
    #[doc(hidden)]
    U64(vec::IntoIter<u64>),
}
#[cfg(feature = "alloc")]
impl<'a, S: Index<usize, Output = T> + ?Sized + 'a, T: 'a> Iterator
for SliceChooseIter<'a, S, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.indices.next().map(|i| &self.slice[i])
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.indices.len(), Some(self.indices.len()))
    }
}
