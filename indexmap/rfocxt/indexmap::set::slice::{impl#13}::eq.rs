use super::{Bucket, Entries, IndexSet, IntoIter, Iter};
use crate::util::{slice_eq, try_simplify_range};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{self, Bound, Index, RangeBounds};
#[repr(transparent)]
pub struct Slice<T> {
    pub(crate) entries: [Bucket<T>],
}
impl<T, const N: usize, U> PartialEq<Slice<U>> for [T; N]
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Slice<U>) -> bool {
        <[T] as PartialEq<Slice<U>>>::eq(self, other)
    }
}
