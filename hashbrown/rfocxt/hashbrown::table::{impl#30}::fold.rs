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
impl<'a, T> Iterator for IterHash<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(bucket) => Some(unsafe { bucket.as_ref() }),
            None => None,
        }
    }
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, bucket| unsafe { f(acc, bucket.as_ref()) })
    }
}
