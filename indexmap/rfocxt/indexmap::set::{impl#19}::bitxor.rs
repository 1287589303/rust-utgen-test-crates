type Bucket<T> = super::Bucket<T, ()>;
pub use self::iter::{
    Difference, Drain, Intersection, IntoIter, Iter, Splice, SymmetricDifference, Union,
};
pub use self::mutable::MutableValues;
pub use self::slice::Slice;
#[cfg(feature = "rayon")]
pub use crate::rayon::set as rayon;
use crate::TryReserveError;
#[cfg(feature = "std")]
use std::collections::hash_map::RandomState;
use crate::util::try_simplify_range;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::ops::{BitAnd, BitOr, BitXor, Index, RangeBounds, Sub};
use super::{Entries, Equivalent, IndexMap};
#[cfg(not(feature = "std"))]
pub struct IndexSet<T, S> {
    pub(crate) map: IndexMap<T, (), S>,
}
pub struct SymmetricDifference<'a, T, S1, S2> {
    iter: Chain<Difference<'a, T, S2>, Difference<'a, T, S1>>,
}
impl<T, S1, S2> BitXor<&IndexSet<T, S2>> for &IndexSet<T, S1>
where
    T: Eq + Hash + Clone,
    S1: BuildHasher + Default,
    S2: BuildHasher,
{
    type Output = IndexSet<T, S1>;
    fn bitxor(self, other: &IndexSet<T, S2>) -> Self::Output {
        self.symmetric_difference(other).cloned().collect()
    }
}
