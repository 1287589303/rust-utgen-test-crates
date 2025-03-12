use super::{Bucket, Entries, IndexSet, Slice};
use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::{Chain, FusedIterator};
use core::ops::RangeBounds;
use core::slice::Iter as SliceIter;
struct UnitValue<I>(I);
impl<I: Iterator> Iterator for UnitValue<I> {
    type Item = (I::Item, ());
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| (x, ()))
    }
}
