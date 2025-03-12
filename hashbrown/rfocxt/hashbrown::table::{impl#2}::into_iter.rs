use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
pub struct HashTable<T, A = Global>
where
    A: Allocator,
{
    pub(crate) raw: RawTable<T, A>,
}
pub struct IntoIter<T, A = Global>
where
    A: Allocator,
{
    inner: RawIntoIter<T, A>,
}
pub struct RawTable<T, A: Allocator = Global> {
    table: RawTableInner,
    alloc: A,
    marker: PhantomData<T>,
}
pub struct RawIntoIter<T, A: Allocator = Global> {
    iter: RawIter<T>,
    allocation: Option<(NonNull<u8>, Layout, A)>,
    marker: PhantomData<T>,
}
pub struct IntoIter<K, V, A: Allocator = Global> {
    inner: RawIntoIter<(K, V), A>,
}
pub struct IntoIter<K, A: Allocator = Global> {
    iter: map::IntoIter<K, (), A>,
}
impl<T, A> IntoIterator for HashTable<T, A>
where
    A: Allocator,
{
    type Item = T;
    type IntoIter = IntoIter<T, A>;
    fn into_iter(self) -> IntoIter<T, A> {
        IntoIter {
            inner: self.raw.into_iter(),
        }
    }
}
