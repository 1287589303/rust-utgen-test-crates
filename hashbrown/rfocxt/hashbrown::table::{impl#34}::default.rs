use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
pub struct IterHashMut<'a, T> {
    inner: RawIterHash<T>,
    marker: PhantomData<&'a mut T>,
}
pub struct RawIterHash<T> {
    inner: RawIterHashInner,
    _marker: PhantomData<T>,
}
impl<T> Default for IterHashMut<'_, T> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        IterHashMut {
            inner: Default::default(),
            marker: PhantomData,
        }
    }
}
