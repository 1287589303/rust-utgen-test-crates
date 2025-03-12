use alloc::{vec, vec::Vec};
use crate::{
    nfa::thompson::Transition,
    util::{
        int::{Usize, U64},
        primitives::StateID,
    },
};
const PRIME: u64 = 1099511628211;
const INIT: u64 = 14695981039346656037;
pub(crate) trait Usize {
    fn as_u8(self) -> u8;
    fn as_u16(self) -> u16;
    fn as_u32(self) -> u32;
    fn as_u64(self) -> u64;
}
pub(crate) trait U64 {
    fn as_usize(self) -> usize;
    fn low_u8(self) -> u8;
    fn low_u16(self) -> u16;
    fn low_u32(self) -> u32;
    fn high_u32(self) -> u32;
}
#[derive(Clone, Debug)]
pub struct Utf8SuffixMap {
    /// The current version of this map. Only entries with matching versions
    /// are considered during lookups. If an entry is found with a mismatched
    /// version, then the map behaves as if the entry does not exist.
    version: u16,
    /// The total number of entries this map can store.
    capacity: usize,
    /// The actual entries, keyed by hash. Collisions between different states
    /// result in the old state being dropped.
    map: Vec<Utf8SuffixEntry>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Utf8SuffixKey {
    pub from: StateID,
    pub start: u8,
    pub end: u8,
}
#[derive(Clone, Debug, Default)]
struct Utf8SuffixEntry {
    /// The version of the map used to produce this entry. If this entry's
    /// version does not match the current version of the map, then the map
    /// should behave as if this entry does not exist.
    version: u16,
    /// The key, which consists of a transition in a particular state.
    key: Utf8SuffixKey,
    /// The identifier that the transition in the key maps to.
    val: StateID,
}
impl Utf8SuffixMap {
    pub fn new(capacity: usize) -> Utf8SuffixMap {}
    pub fn clear(&mut self) {}
    pub fn hash(&self, key: &Utf8SuffixKey) -> usize {
        const PRIME: u64 = 1099511628211;
        const INIT: u64 = 14695981039346656037;
        let mut h = INIT;
        h = (h ^ key.from.as_u64()).wrapping_mul(PRIME);
        h = (h ^ u64::from(key.start)).wrapping_mul(PRIME);
        h = (h ^ u64::from(key.end)).wrapping_mul(PRIME);
        (h % self.map.len().as_u64()).as_usize()
    }
    pub fn get(&mut self, key: &Utf8SuffixKey, hash: usize) -> Option<StateID> {}
    pub fn set(&mut self, key: Utf8SuffixKey, hash: usize, state_id: StateID) {}
}
