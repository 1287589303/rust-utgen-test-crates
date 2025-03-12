use crate::Rng;
#[cfg(feature = "alloc")]
use alloc::string::String;
use core::iter;
#[derive(Debug)]
pub struct Iter<D, R, T> {
    distr: D,
    rng: R,
    phantom: core::marker::PhantomData<T>,
}
impl<D, R, T> Iterator for Iter<D, R, T>
where
    D: Distribution<T>,
    R: Rng,
{
    type Item = T;
    #[inline(always)]
    fn next(&mut self) -> Option<T> {}
    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}
