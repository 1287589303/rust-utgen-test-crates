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
#[derive(Clone, Debug, Default)]
pub struct GroupInfo(Arc<GroupInfoInner>);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Debug, Default)]
struct GroupInfoInner {
    slot_ranges: Vec<(SmallIndex, SmallIndex)>,
    name_to_index: Vec<CaptureNameMap>,
    index_to_name: Vec<Vec<Option<Arc<str>>>>,
    memory_extra: usize,
}
#[derive(Debug)]
pub struct GroupInfoAllNames<'a> {
    group_info: &'a GroupInfo,
    pids: PatternIDIter,
    current_pid: Option<PatternID>,
    names: Option<core::iter::Enumerate<GroupInfoPatternNames<'a>>>,
}
#[derive(Clone, Debug)]
pub struct GroupInfoPatternNames<'a> {
    it: core::slice::Iter<'a, Option<Arc<str>>>,
}
impl GroupInfo {
    pub fn new<P, G, N>(pattern_groups: P) -> Result<GroupInfo, GroupInfoError>
    where
        P: IntoIterator<Item = G>,
        G: IntoIterator<Item = Option<N>>,
        N: AsRef<str>,
    {}
    pub fn empty() -> GroupInfo {}
    #[inline]
    pub fn to_index(&self, pid: PatternID, name: &str) -> Option<usize> {}
    #[inline]
    pub fn to_name(&self, pid: PatternID, group_index: usize) -> Option<&str> {}
    #[inline]
    pub fn pattern_names(&self, pid: PatternID) -> GroupInfoPatternNames<'_> {}
    #[inline]
    pub fn all_names(&self) -> GroupInfoAllNames<'_> {
        GroupInfoAllNames {
            group_info: self,
            pids: PatternID::iter(self.pattern_len()),
            current_pid: None,
            names: None,
        }
    }
    #[inline]
    pub fn slots(&self, pid: PatternID, group_index: usize) -> Option<(usize, usize)> {}
    #[inline]
    pub fn slot(&self, pid: PatternID, group_index: usize) -> Option<usize> {}
    #[inline]
    pub fn pattern_len(&self) -> usize {
        self.0.pattern_len()
    }
    #[inline]
    pub fn group_len(&self, pid: PatternID) -> usize {}
    #[inline]
    pub fn all_group_len(&self) -> usize {}
    #[inline]
    pub fn slot_len(&self) -> usize {}
    #[inline]
    pub fn implicit_slot_len(&self) -> usize {}
    #[inline]
    pub fn explicit_slot_len(&self) -> usize {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
