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
    ) -> Result<(), GroupInfoError> {
        let end = &mut self.slot_ranges[pid].1;
        *end = SmallIndex::new(end.as_usize() + 2)
            .map_err(|_| { GroupInfoError::too_many_groups(pid, group.as_usize()) })?;
        if let Some(name) = maybe_name {
            let name = Arc::<str>::from(name.as_ref());
            if self.name_to_index[pid].contains_key(&*name) {
                return Err(GroupInfoError::duplicate(pid, &name));
            }
            let len = name.len();
            self.name_to_index[pid].insert(Arc::clone(&name), group);
            self.index_to_name[pid].push(Some(name));
            self.memory_extra += 2 * (len + core::mem::size_of::<Option<Arc<str>>>());
            self.memory_extra += core::mem::size_of::<SmallIndex>();
        } else {
            self.index_to_name[pid].push(None);
            self.memory_extra += core::mem::size_of::<Option<Arc<str>>>();
        }
        assert_eq!(group.one_more(), self.group_len(pid));
        assert_eq!(group.one_more(), self.index_to_name[pid].len());
        Ok(())
    }
    fn fixup_slot_ranges(&mut self) -> Result<(), GroupInfoError> {}
    fn pattern_len(&self) -> usize {}
    fn group_len(&self, pid: PatternID) -> usize {
        let (start, end) = match self.slot_ranges.get(pid.as_usize()) {
            None => return 0,
            Some(range) => range,
        };
        1 + ((end.as_usize() - start.as_usize()) / 2)
    }
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
    pub fn one_more(&self) -> usize {
        self.as_usize() + 1
    }
    #[inline]
    pub fn from_ne_bytes(bytes: [u8; 4]) -> Result<SmallIndex, SmallIndexError> {}
    #[inline]
    pub fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> SmallIndex {}
    #[inline]
    pub fn to_ne_bytes(&self) -> [u8; 4] {}
}
impl GroupInfoError {
    fn too_many_patterns(err: PatternIDError) -> GroupInfoError {}
    fn too_many_groups(pattern: PatternID, minimum: usize) -> GroupInfoError {}
    fn missing_groups(pattern: PatternID) -> GroupInfoError {}
    fn first_must_be_unnamed(pattern: PatternID) -> GroupInfoError {}
    fn duplicate(pattern: PatternID, name: &str) -> GroupInfoError {
        GroupInfoError {
            kind: GroupInfoErrorKind::Duplicate {
                pattern,
                name: String::from(name),
            },
        }
    }
}
