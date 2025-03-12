use crate::raw::{
    Allocator, Bucket, Global, RawDrain, RawExtractIf, RawIntoIter, RawIter, RawTable,
};
use crate::{DefaultHashBuilder, Equivalent, TryReserveError};
use core::borrow::Borrow;
use core::fmt::{self, Debug};
use core::hash::{BuildHasher, Hash};
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::mem;
use core::ops::Index;
#[cfg(feature = "raw-entry")]
pub use crate::raw_entry::*;
#[allow(clippy::missing_safety_doc)]
pub unsafe trait Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
}
#[must_use = "Iterators are lazy unless consumed"]
pub struct ExtractIf<'a, K, V, F, A: Allocator = Global>
where
    F: FnMut(&K, &mut V) -> bool,
{
    f: F,
    inner: RawExtractIf<'a, (K, V), A>,
}
pub(crate) struct RawExtractIf<'a, T, A: Allocator> {
    pub iter: RawIter<T>,
    pub table: &'a mut RawTable<T, A>,
}
impl<K, V, F, A> Iterator for ExtractIf<'_, K, V, F, A>
where
    F: FnMut(&K, &mut V) -> bool,
    A: Allocator,
{
    type Item = (K, V);
    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next(|&mut (ref k, ref mut v)| (self.f)(k, v))
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {}
}
