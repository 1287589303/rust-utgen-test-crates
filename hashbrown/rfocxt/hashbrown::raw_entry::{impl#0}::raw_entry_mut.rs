use crate::hash_map::{equivalent, make_hash, make_hasher};
use crate::raw::{Allocator, Bucket, Global, RawTable};
use crate::{Equivalent, HashMap};
use core::fmt::{self, Debug};
use core::hash::{BuildHasher, Hash};
use core::mem;
#[allow(clippy::missing_safety_doc)]
pub unsafe trait Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
}
pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) hash_builder: S,
    pub(crate) table: RawTable<(K, V), A>,
}
pub struct RawEntryBuilderMut<'a, K, V, S, A: Allocator = Global> {
    map: &'a mut HashMap<K, V, S, A>,
}
pub struct RawTable<T, A: Allocator = Global> {
    table: RawTableInner,
    alloc: A,
    marker: PhantomData<T>,
}
impl<K, V, S, A: Allocator> HashMap<K, V, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn raw_entry_mut(&mut self) -> RawEntryBuilderMut<'_, K, V, S, A> {
        RawEntryBuilderMut { map: self }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn raw_entry(&self) -> RawEntryBuilder<'_, K, V, S, A> {}
}
