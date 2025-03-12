#[cfg(feature = "std")]
type CaptureNameMap = std::collections::HashMap<Arc<str>, SmallIndex>;
#[cfg(not(feature = "std"))]
type CaptureNameMap = alloc::collections::BTreeMap<Arc<str>, SmallIndex>;
use alloc::{string::String, sync::Arc, vec, vec::Vec};
use crate::util::{
    interpolate,
    primitives::{NonMaxUsize, PatternID, PatternIDError, PatternIDIter, SmallIndex},
    search::{Match, Span},
};
#[derive(Clone)]
pub struct Captures {
    /// The group info that these capture groups are coupled to. This is what
    /// gives the "convenience" of the `Captures` API. Namely, it provides the
    /// slot mapping and the name|-->index mapping for capture lookups by name.
    group_info: GroupInfo,
    /// The ID of the pattern that matched. Regex engines must set this to
    /// None when no match occurs.
    pid: Option<PatternID>,
    /// The slot values, i.e., submatch offsets.
    ///
    /// In theory, the smallest sequence of slots would be something like
    /// `max(groups(pattern) for pattern in regex) * 2`, but instead, we use
    /// `sum(groups(pattern) for pattern in regex) * 2`. Why?
    ///
    /// Well, the former could be used in theory, because we don't generally
    /// have any overlapping APIs that involve capturing groups. Therefore,
    /// there's technically never any need to have slots set for multiple
    /// patterns. However, this might change some day, in which case, we would
    /// need to have slots available.
    ///
    /// The other reason is that during the execution of some regex engines,
    /// there exists a point in time where multiple slots for different
    /// patterns may be written to before knowing which pattern has matched.
    /// Therefore, the regex engines themselves, in order to support multiple
    /// patterns correctly, must have all slots available. If `Captures`
    /// doesn't have all slots available, then regex engines can't write
    /// directly into the caller provided `Captures` and must instead write
    /// into some other storage and then copy the slots involved in the match
    /// at the end of the search.
    ///
    /// So overall, at least as of the time of writing, it seems like the path
    /// of least resistance is to just require allocating all possible slots
    /// instead of the conceptual minimum. Another way to justify this is that
    /// the most common case is a single pattern, in which case, there is no
    /// inefficiency here since the 'max' and 'sum' calculations above are
    /// equivalent in that case.
    ///
    /// N.B. The mapping from group index to slot is maintained by `GroupInfo`
    /// and is considered an API guarantee. See `GroupInfo` for more details on
    /// that mapping.
    ///
    /// N.B. `Option<NonMaxUsize>` has the same size as a `usize`.
    slots: Vec<Option<NonMaxUsize>>,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Match {
    /// The pattern ID.
    pattern: PatternID,
    /// The underlying match span.
    span: Span,
}
#[derive(Clone, Debug, Default)]
pub struct GroupInfo(Arc<GroupInfoInner>);
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct NonMaxUsize(NonZeroUsize);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Span {
    /// The start offset of the span, inclusive.
    pub start: usize,
    /// The end offset of the span, exclusive.
    pub end: usize,
}
impl Captures {
    pub fn all(group_info: GroupInfo) -> Captures {}
    pub fn matches(group_info: GroupInfo) -> Captures {}
    pub fn empty(group_info: GroupInfo) -> Captures {}
    #[inline]
    pub fn is_match(&self) -> bool {}
    #[inline]
    pub fn pattern(&self) -> Option<PatternID> {
        self.pid
    }
    #[inline]
    pub fn get_match(&self) -> Option<Match> {
        Some(Match::new(self.pattern()?, self.get_group(0)?))
    }
    #[inline]
    pub fn get_group(&self, index: usize) -> Option<Span> {
        let pid = self.pattern()?;
        let (slot_start, slot_end) = if self.group_info().pattern_len() == 1 {
            (index.checked_mul(2)?, index.checked_mul(2)?.checked_add(1)?)
        } else {
            self.group_info().slots(pid, index)?
        };
        let start = self.slots.get(slot_start).copied()??;
        let end = self.slots.get(slot_end).copied()??;
        Some(Span {
            start: start.get(),
            end: end.get(),
        })
    }
    pub fn get_group_by_name(&self, name: &str) -> Option<Span> {}
    pub fn iter(&self) -> CapturesPatternIter<'_> {}
    pub fn group_len(&self) -> usize {}
    pub fn group_info(&self) -> &GroupInfo {}
    pub fn interpolate_string(&self, haystack: &str, replacement: &str) -> String {}
    pub fn interpolate_string_into(
        &self,
        haystack: &str,
        replacement: &str,
        dst: &mut String,
    ) {}
    pub fn interpolate_bytes(&self, haystack: &[u8], replacement: &[u8]) -> Vec<u8> {}
    pub fn interpolate_bytes_into(
        &self,
        haystack: &[u8],
        replacement: &[u8],
        dst: &mut Vec<u8>,
    ) {}
    pub fn extract<'h, const N: usize>(
        &self,
        haystack: &'h str,
    ) -> (&'h str, [&'h str; N]) {}
    pub fn extract_bytes<'h, const N: usize>(
        &self,
        haystack: &'h [u8],
    ) -> (&'h [u8], [&'h [u8]; N]) {}
}
impl Match {
    #[inline]
    pub fn new<S: Into<Span>>(pattern: PatternID, span: S) -> Match {
        let span: Span = span.into();
        assert!(span.start <= span.end, "invalid match span");
        Match { pattern, span }
    }
    #[inline]
    pub fn must<S: Into<Span>>(pattern: usize, span: S) -> Match {}
    #[inline]
    pub fn pattern(&self) -> PatternID {}
    #[inline]
    pub fn start(&self) -> usize {}
    #[inline]
    pub fn end(&self) -> usize {}
    #[inline]
    pub fn range(&self) -> core::ops::Range<usize> {}
    #[inline]
    pub fn span(&self) -> Span {}
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn len(&self) -> usize {}
}
