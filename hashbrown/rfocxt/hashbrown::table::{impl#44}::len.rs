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
impl<T, A: Allocator> ExactSizeIterator for Drain<'_, T, A> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}
