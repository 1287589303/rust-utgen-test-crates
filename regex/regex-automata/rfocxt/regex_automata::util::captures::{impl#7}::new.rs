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
#[derive(Debug, Default)]
struct GroupInfoInner {
    slot_ranges: Vec<(SmallIndex, SmallIndex)>,
    name_to_index: Vec<CaptureNameMap>,
    index_to_name: Vec<Vec<Option<Arc<str>>>>,
    memory_extra: usize,
}
#[derive(Clone, Debug)]
pub struct GroupInfoError {
    kind: GroupInfoErrorKind,
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SmallIndex(u32);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmallIndexError {
    attempted: u64,
}
impl GroupInfo {
    pub fn new<P, G, N>(pattern_groups: P) -> Result<GroupInfo, GroupInfoError>
    where
        P: IntoIterator<Item = G>,
        G: IntoIterator<Item = Option<N>>,
        N: AsRef<str>,
    {
        let mut group_info = GroupInfoInner {
            slot_ranges: vec![],
            name_to_index: vec![],
            index_to_name: vec![],
            memory_extra: 0,
        };
        for (pattern_index, groups) in pattern_groups.into_iter().enumerate() {
            let pid = PatternID::new(pattern_index)
                .map_err(GroupInfoError::too_many_patterns)?;
            let mut groups_iter = groups.into_iter().enumerate();
            match groups_iter.next() {
                None => return Err(GroupInfoError::missing_groups(pid)),
                Some((_, Some(_))) => {
                    return Err(GroupInfoError::first_must_be_unnamed(pid));
                }
                Some((_, None)) => {}
            }
            group_info.add_first_group(pid);
            for (group_index, maybe_name) in groups_iter {
                let group = SmallIndex::new(group_index)
                    .map_err(|_| { GroupInfoError::too_many_groups(pid, group_index) })?;
                group_info.add_explicit_group(pid, group, maybe_name)?;
            }
        }
        group_info.fixup_slot_ranges()?;
        Ok(GroupInfo(Arc::new(group_info)))
    }
    pub fn empty() -> GroupInfo {}
    #[inline]
    pub fn to_index(&self, pid: PatternID, name: &str) -> Option<usize> {}
    #[inline]
    pub fn to_name(&self, pid: PatternID, group_index: usize) -> Option<&str> {}
    #[inline]
    pub fn pattern_names(&self, pid: PatternID) -> GroupInfoPatternNames<'_> {}
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
impl GroupInfoInner {
    fn add_first_group(&mut self, pid: PatternID) {
        assert_eq!(pid.as_usize(), self.slot_ranges.len());
        assert_eq!(pid.as_usize(), self.name_to_index.len());
        assert_eq!(pid.as_usize(), self.index_to_name.len());
        let slot_start = self.small_slot_len();
        self.slot_ranges.push((slot_start, slot_start));
        self.name_to_index.push(CaptureNameMap::new());
        self.index_to_name.push(vec![None]);
        self.memory_extra += core::mem::size_of::<Option<Arc<str>>>();
    }
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
    fn pattern_len(&self) -> usize {}
    fn group_len(&self, pid: PatternID) -> usize {}
    fn small_slot_len(&self) -> SmallIndex {}
}
impl GroupInfoError {
    fn too_many_patterns(err: PatternIDError) -> GroupInfoError {}
    fn too_many_groups(pattern: PatternID, minimum: usize) -> GroupInfoError {}
    fn missing_groups(pattern: PatternID) -> GroupInfoError {
        GroupInfoError {
            kind: GroupInfoErrorKind::MissingGroups {
                pattern,
            },
        }
    }
    fn first_must_be_unnamed(pattern: PatternID) -> GroupInfoError {
        GroupInfoError {
            kind: GroupInfoErrorKind::FirstMustBeUnnamed {
                pattern,
            },
        }
    }
    fn duplicate(pattern: PatternID, name: &str) -> GroupInfoError {}
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
    pub const fn as_usize(&self) -> usize {}
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
