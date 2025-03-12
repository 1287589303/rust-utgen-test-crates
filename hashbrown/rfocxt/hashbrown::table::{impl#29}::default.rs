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
impl<T> Default for IterHash<'_, T> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        IterHash {
            inner: Default::default(),
            marker: PhantomData,
        }
    }
}
