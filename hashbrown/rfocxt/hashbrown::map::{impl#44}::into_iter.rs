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
pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) hash_builder: S,
    pub(crate) table: RawTable<(K, V), A>,
}
pub struct IterMut<'a, K, V> {
    inner: RawIter<(K, V)>,
    marker: PhantomData<(&'a K, &'a mut V)>,
}
pub struct IterMut<'a, T> {
    inner: RawIter<T>,
    marker: PhantomData<&'a mut T>,
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
impl<K, V, S, A: Allocator> HashMap<K, V, S, A> {
    #[inline]
    pub fn allocator(&self) -> &A {}
    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg_attr(feature = "rustc-dep-of-std", rustc_const_stable_indirect)]
    pub const fn with_hasher_in(hash_builder: S, alloc: A) -> Self {
        Self {
            hash_builder,
            table: RawTable::new_in(alloc),
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn with_capacity_and_hasher_in(
        capacity: usize,
        hash_builder: S,
        alloc: A,
    ) -> Self {
        Self {
            hash_builder,
            table: RawTable::with_capacity_in(capacity, alloc),
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn hasher(&self) -> &S {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn capacity(&self) -> usize {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn keys(&self) -> Keys<'_, K, V> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn values(&self) -> Values<'_, K, V> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn iter(&self) -> Iter<'_, K, V> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        unsafe {
            IterMut {
                inner: self.table.iter(),
                marker: PhantomData,
            }
        }
    }
    #[cfg(test)]
    #[cfg_attr(feature = "inline-more", inline)]
    fn raw_capacity(&self) -> usize {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn len(&self) -> usize {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn is_empty(&self) -> bool {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn drain(&mut self) -> Drain<'_, K, V, A> {}
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn extract_if<F>(&mut self, f: F) -> ExtractIf<'_, K, V, F, A>
    where
        F: FnMut(&K, &mut V) -> bool,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn clear(&mut self) {}
    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V, A> {}
    #[inline]
    pub fn into_values(self) -> IntoValues<K, V, A> {}
}
