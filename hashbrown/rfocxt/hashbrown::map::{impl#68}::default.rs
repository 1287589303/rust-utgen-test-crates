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
pub struct ValuesMut<'a, K, V> {
    inner: IterMut<'a, K, V>,
}
pub struct IterMut<'a, T> {
    inner: RawIter<T>,
    marker: PhantomData<&'a mut T>,
}
pub struct IterMut<'a, K, V> {
    inner: RawIter<(K, V)>,
    marker: PhantomData<(&'a K, &'a mut V)>,
}
impl<K, V> Default for ValuesMut<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}
