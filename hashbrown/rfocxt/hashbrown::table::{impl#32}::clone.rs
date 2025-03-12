use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
pub struct IterHash<'a, T> {
    inner: RawIterHash<T>,
    marker: PhantomData<&'a T>,
}
pub struct RawIterHash<T> {
    inner: RawIterHashInner,
    _marker: PhantomData<T>,
}
impl<'a, T> Clone for IterHash<'a, T> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> IterHash<'a, T> {
        IterHash {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }
}
