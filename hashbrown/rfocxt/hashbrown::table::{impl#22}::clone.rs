use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
pub struct Iter<'a, T> {
    inner: RawIter<T>,
    marker: PhantomData<&'a T>,
}
pub struct RawIter<T> {
    pub(crate) iter: RawIterRange<T>,
    items: usize,
}
impl<'a, T> Clone for Iter<'a, T> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Iter<'a, T> {
        Iter {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }
}
