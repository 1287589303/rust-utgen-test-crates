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
#[derive(Debug, Default)]
struct GroupInfoInner {
    slot_ranges: Vec<(SmallIndex, SmallIndex)>,
    name_to_index: Vec<CaptureNameMap>,
    index_to_name: Vec<Vec<Option<Arc<str>>>>,
    memory_extra: usize,
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SmallIndex(u32);
#[derive(Clone, Debug)]
pub struct GroupInfoError {
    kind: GroupInfoErrorKind,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmallIndexError {
    attempted: u64,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
impl GroupInfoInner {
    fn add_first_group(&mut self, pid: PatternID) {}
    fn add_explicit_group<N: AsRef<str>>(
        &mut self,
        pid: PatternID,
        group: SmallIndex,
        maybe_name: Option<N>,
    ) -> Result<(), GroupInfoError> {}
    fn fixup_slot_ranges(&mut self) -> Result<(), GroupInfoError> {
        use crate::util::primitives::IteratorIndexExt;
        let offset = self.pattern_len().checked_mul(2).unwrap();
        for (pid, &mut (ref mut start, ref mut end)) in self
            .slot_ranges
            .iter_mut()
            .with_pattern_ids()
        {
            let group_len = 1 + ((end.as_usize() - start.as_usize()) / 2);
            let new_end = match end.as_usize().checked_add(offset) {
                Some(new_end) => new_end,
                None => return Err(GroupInfoError::too_many_groups(pid, group_len)),
            };
            *end = SmallIndex::new(new_end)
                .map_err(|_| { GroupInfoError::too_many_groups(pid, group_len) })?;
            *start = SmallIndex::new(start.as_usize() + offset).unwrap();
        }
        Ok(())
    }
    fn pattern_len(&self) -> usize {
        self.slot_ranges.len()
    }
    fn group_len(&self, pid: PatternID) -> usize {}
    fn small_slot_len(&self) -> SmallIndex {}
}
impl SmallIndex {
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    pub const MAX: SmallIndex = SmallIndex::new_unchecked(core::i32::MAX as usize - 1);
    #[cfg(target_pointer_width = "16")]
    pub const MAX: SmallIndex = SmallIndex::new_unchecked(core::isize::MAX - 1);
    pub const LIMIT: usize = SmallIndex::MAX.as_usize() + 1;
    pub const ZERO: SmallIndex = SmallIndex::new_unchecked(0);
    pub const SIZE: usize = core::mem::size_of::<SmallIndex>();
    #[inline]
    pub fn new(index: usize) -> Result<SmallIndex, SmallIndexError> {
        SmallIndex::try_from(index)
    }
    #[inline]
    pub const fn new_unchecked(index: usize) -> SmallIndex {}
    #[inline]
    pub fn must(index: usize) -> SmallIndex {}
    #[inline]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }
    #[inline]
    pub const fn as_u64(&self) -> u64 {}
    #[inline]
    pub const fn as_u32(&self) -> u32 {}
    #[inline]
    pub const fn as_i32(&self) -> i32 {}
    #[inline]
    pub fn one_more(&self) -> usize {}
    #[inline]
    pub fn from_ne_bytes(bytes: [u8; 4]) -> Result<SmallIndex, SmallIndexError> {}
    #[inline]
    pub fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> SmallIndex {}
    #[inline]
    pub fn to_ne_bytes(&self) -> [u8; 4] {}
}
impl GroupInfoError {
    fn too_many_patterns(err: PatternIDError) -> GroupInfoError {}
    fn too_many_groups(pattern: PatternID, minimum: usize) -> GroupInfoError {
        GroupInfoError {
            kind: GroupInfoErrorKind::TooManyGroups {
                pattern,
                minimum,
            },
        }
    }
    fn missing_groups(pattern: PatternID) -> GroupInfoError {}
    fn first_must_be_unnamed(pattern: PatternID) -> GroupInfoError {}
    fn duplicate(pattern: PatternID, name: &str) -> GroupInfoError {}
}
