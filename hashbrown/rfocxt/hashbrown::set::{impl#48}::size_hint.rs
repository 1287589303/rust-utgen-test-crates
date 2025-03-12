use crate::{Equivalent, TryReserveError};
use core::hash::{BuildHasher, Hash};
use core::iter::{Chain, FusedIterator};
use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign,
};
use core::{fmt, mem};
use map::make_hash;
use super::map::{self, HashMap, Keys};
use crate::raw::{Allocator, Global, RawExtractIf};
use crate::DefaultHashBuilder;
#[allow(clippy::missing_safety_doc)]
pub unsafe trait Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
}
pub struct Difference<'a, T, S, A: Allocator = Global> {
    iter: Iter<'a, T>,
    other: &'a HashSet<T, S, A>,
}
pub struct HashSet<T, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) map: HashMap<T, (), S, A>,
}
pub struct Iter<'a, K> {
    iter: Keys<'a, K, ()>,
}
pub struct Iter<'a, T> {
    inner: RawIter<T>,
    marker: PhantomData<&'a T>,
}
pub struct Iter<'a, K, V> {
    inner: RawIter<(K, V)>,
    marker: PhantomData<(&'a K, &'a V)>,
}
impl<'a, T, S, A> Iterator for Difference<'a, T, S, A>
where
    T: Eq + Hash,
    S: BuildHasher,
    A: Allocator,
{
    type Item = &'a T;
    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<&'a T> {}
    #[cfg_attr(feature = "inline-more", inline)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        (lower.saturating_sub(self.other.len()), upper)
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {}
}
impl<T, S, A: Allocator> HashSet<T, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn capacity(&self) -> usize {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn iter(&self) -> Iter<'_, T> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn len(&self) -> usize {
        self.map.len()
    }
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
