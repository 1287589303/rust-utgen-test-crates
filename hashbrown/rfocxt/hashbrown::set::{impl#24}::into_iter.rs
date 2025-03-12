use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
pub(crate) trait TagSliceExt {
    fn fill_tag(&mut self, tag: Tag);
    #[inline]
    fn fill_empty(&mut self);
}
#[cfg(not(feature = "equivalent"))]
pub trait Equivalent<K: ?Sized> {
    fn equivalent(&self, key: &K) -> bool;
}
#[allow(clippy::missing_safety_doc)]
pub unsafe trait Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
}
pub struct HashSet<T, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) map: HashMap<T, (), S, A>,
}
pub struct Iter<'a, K> {
    iter: Keys<'a, K, ()>,
}
pub struct Iter<'a, K, V> {
    inner: RawIter<(K, V)>,
    marker: PhantomData<(&'a K, &'a V)>,
}
pub struct Iter<'a, T> {
    inner: RawIter<T>,
    marker: PhantomData<&'a T>,
}
impl<'a, T, A> IntoIterator for &'a mut HashTable<T, A>
where
    A: Allocator,
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}
impl<T, S, A: Allocator> HashSet<T, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn capacity(&self) -> usize {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { iter: self.map.keys() }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn len(&self) -> usize {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn is_empty(&self) -> bool {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn drain(&mut self) -> Drain<'_, T, A> {}
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn extract_if<F>(&mut self, f: F) -> ExtractIf<'_, T, F, A>
    where
        F: FnMut(&T) -> bool,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn clear(&mut self) {}
}
