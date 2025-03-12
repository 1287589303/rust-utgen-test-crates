use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
#[allow(clippy::missing_safety_doc)]
pub unsafe trait Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
}
pub struct Drain<'a, T, A: Allocator = Global> {
    inner: RawDrain<'a, T, A>,
}
pub struct RawDrain<'a, T, A: Allocator = Global> {
    iter: RawIter<T>,
    table: RawTableInner,
    orig_table: NonNull<RawTableInner>,
    marker: PhantomData<&'a RawTable<T, A>>,
}
impl<T, A: Allocator> Iterator for Drain<'_, T, A> {
    type Item = T;
    fn next(&mut self) -> Option<T> {}
    fn size_hint(&self) -> (usize, Option<usize>) {}
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, f)
    }
}
