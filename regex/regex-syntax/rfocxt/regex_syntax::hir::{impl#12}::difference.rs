use core::{char, cmp};
use alloc::{
    boxed::Box, format, string::{String, ToString},
    vec, vec::Vec,
};
use crate::{
    ast::Span, hir::interval::{Interval, IntervalSet, IntervalSetIter},
    unicode,
};
pub use crate::{
    hir::visitor::{visit, Visitor},
    unicode::CaseFoldError,
};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
}
#[derive(Clone, Debug)]
pub struct IntervalSet<I> {
    /// A sorted set of non-overlapping ranges.
    ranges: Vec<I>,
    /// While not required at all for correctness, we keep track of whether an
    /// interval set has been case folded or not. This helps us avoid doing
    /// redundant work if, for example, a set has already been cased folded.
    /// And note that whether a set is folded or not is preserved through
    /// all of the pairwise set operations. That is, if both interval sets
    /// have been case folded, then any of difference, union, intersection or
    /// symmetric difference all produce a case folded set.
    ///
    /// Note that when this is true, it *must* be the case that the set is case
    /// folded. But when it's false, the set *may* be case folded. In other
    /// words, we only set this to true when we know it to be case, but we're
    /// okay with it being false if it would otherwise be costly to determine
    /// whether it should be true. This means code cannot assume that a false
    /// value necessarily indicates that the set is not case folded.
    ///
    /// Bottom line: this is a performance optimization.
    folded: bool,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClassUnicodeRange {
    start: char,
    end: char,
}
impl ClassUnicode {
    pub fn new<I>(ranges: I) -> ClassUnicode
    where
        I: IntoIterator<Item = ClassUnicodeRange>,
    {}
    pub fn empty() -> ClassUnicode {}
    pub fn push(&mut self, range: ClassUnicodeRange) {}
    pub fn iter(&self) -> ClassUnicodeIter<'_> {}
    pub fn ranges(&self) -> &[ClassUnicodeRange] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn try_case_fold_simple(&mut self) -> core::result::Result<(), CaseFoldError> {}
    pub fn negate(&mut self) {}
    pub fn union(&mut self, other: &ClassUnicode) {}
    pub fn intersect(&mut self, other: &ClassUnicode) {}
    pub fn difference(&mut self, other: &ClassUnicode) {
        self.set.difference(&other.set);
    }
    pub fn symmetric_difference(&mut self, other: &ClassUnicode) {}
    pub fn is_ascii(&self) -> bool {}
    pub fn minimum_len(&self) -> Option<usize> {}
    pub fn maximum_len(&self) -> Option<usize> {}
    pub fn literal(&self) -> Option<Vec<u8>> {}
    pub fn to_byte_class(&self) -> Option<ClassBytes> {}
}
impl<I: Interval> IntervalSet<I> {
    pub fn new<T: IntoIterator<Item = I>>(intervals: T) -> IntervalSet<I> {}
    pub fn push(&mut self, interval: I) {}
    pub fn iter(&self) -> IntervalSetIter<'_, I> {}
    pub fn intervals(&self) -> &[I] {}
    pub fn case_fold_simple(&mut self) -> Result<(), unicode::CaseFoldError> {}
    pub fn union(&mut self, other: &IntervalSet<I>) {}
    pub fn intersect(&mut self, other: &IntervalSet<I>) {}
    pub fn difference(&mut self, other: &IntervalSet<I>) {
        if self.ranges.is_empty() || other.ranges.is_empty() {
            return;
        }
        let drain_end = self.ranges.len();
        let (mut a, mut b) = (0, 0);
        'LOOP: while a < drain_end && b < other.ranges.len() {
            if other.ranges[b].upper() < self.ranges[a].lower() {
                b += 1;
                continue;
            }
            if self.ranges[a].upper() < other.ranges[b].lower() {
                let range = self.ranges[a];
                self.ranges.push(range);
                a += 1;
                continue;
            }
            assert!(! self.ranges[a].is_intersection_empty(& other.ranges[b]));
            let mut range = self.ranges[a];
            while b < other.ranges.len()
                && !range.is_intersection_empty(&other.ranges[b])
            {
                let old_range = range;
                range = match range.difference(&other.ranges[b]) {
                    (None, None) => {
                        a += 1;
                        continue 'LOOP;
                    }
                    (Some(range1), None) | (None, Some(range1)) => range1,
                    (Some(range1), Some(range2)) => {
                        self.ranges.push(range1);
                        range2
                    }
                };
                if other.ranges[b].upper() > old_range.upper() {
                    break;
                }
                b += 1;
            }
            self.ranges.push(range);
            a += 1;
        }
        while a < drain_end {
            let range = self.ranges[a];
            self.ranges.push(range);
            a += 1;
        }
        self.ranges.drain(..drain_end);
        self.folded = self.folded && other.folded;
    }
    pub fn symmetric_difference(&mut self, other: &IntervalSet<I>) {}
    pub fn negate(&mut self) {}
    fn canonicalize(&mut self) {}
    fn is_canonical(&self) -> bool {}
}
