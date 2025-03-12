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
#[derive(Debug)]
pub struct GroupInfoAllNames<'a> {
    group_info: &'a GroupInfo,
    pids: PatternIDIter,
    current_pid: Option<PatternID>,
    names: Option<core::iter::Enumerate<GroupInfoPatternNames<'a>>>,
}
#[derive(Clone, Debug, Default)]
pub struct GroupInfo(Arc<GroupInfoInner>);
#[derive(Debug, Default)]
struct GroupInfoInner {
    slot_ranges: Vec<(SmallIndex, SmallIndex)>,
    name_to_index: Vec<CaptureNameMap>,
    index_to_name: Vec<Vec<Option<Arc<str>>>>,
    memory_extra: usize,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Debug)]
pub struct GroupInfoPatternNames<'a> {
    it: core::slice::Iter<'a, Option<Arc<str>>>,
}
impl<'a> Iterator for GroupInfoAllNames<'a> {
    type Item = (PatternID, usize, Option<&'a str>);
    fn next(&mut self) -> Option<(PatternID, usize, Option<&'a str>)> {
        if self.group_info.0.index_to_name.is_empty() {
            return None;
        }
        if self.current_pid.is_none() {
            self.current_pid = Some(self.pids.next()?);
        }
        let pid = self.current_pid.unwrap();
        if self.names.is_none() {
            self.names = Some(self.group_info.pattern_names(pid).enumerate());
        }
        let (group_index, name) = match self.names.as_mut().unwrap().next() {
            Some((group_index, name)) => (group_index, name),
            None => {
                self.current_pid = None;
                self.names = None;
                return self.next();
            }
        };
        Some((pid, group_index, name))
    }
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
    pub fn pattern_names(&self, pid: PatternID) -> GroupInfoPatternNames<'_> {
        GroupInfoPatternNames {
            it: self
                .0
                .index_to_name
                .get(pid.as_usize())
                .map(|indices| indices.iter())
                .unwrap_or([].iter()),
        }
    }
    #[inline]
    pub fn all_names(&self) -> GroupInfoAllNames<'_> {}
    #[inline]
    pub fn slots(&self, pid: PatternID, group_index: usize) -> Option<(usize, usize)> {}
    #[inline]
    pub fn slot(&self, pid: PatternID, group_index: usize) -> Option<usize> {}
    #[inline]
    pub fn pattern_len(&self) -> usize {}
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
