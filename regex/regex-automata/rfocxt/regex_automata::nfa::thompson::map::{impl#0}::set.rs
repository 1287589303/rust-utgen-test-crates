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
#[derive(Clone, Debug)]
pub struct Utf8BoundedMap {
    /// The current version of this map. Only entries with matching versions
    /// are considered during lookups. If an entry is found with a mismatched
    /// version, then the map behaves as if the entry does not exist.
    ///
    /// This makes it possible to clear the map by simply incrementing the
    /// version number instead of actually deallocating any storage.
    version: u16,
    /// The total number of entries this map can store.
    capacity: usize,
    /// The actual entries, keyed by hash. Collisions between different states
    /// result in the old state being dropped.
    map: Vec<Utf8BoundedEntry>,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Transition {
    /// The inclusive start of the byte range.
    pub start: u8,
    /// The inclusive end of the byte range.
    pub end: u8,
    /// The identifier of the state to transition to.
    pub next: StateID,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Debug, Default)]
struct Utf8BoundedEntry {
    /// The version of the map used to produce this entry. If this entry's
    /// version does not match the current version of the map, then the map
    /// should behave as if this entry does not exist.
    version: u16,
    /// The key, which is a sorted sequence of non-overlapping NFA transitions.
    key: Vec<Transition>,
    /// The state ID corresponding to the state containing the transitions in
    /// this entry.
    val: StateID,
}
impl Utf8BoundedMap {
    pub fn new(capacity: usize) -> Utf8BoundedMap {}
    pub fn clear(&mut self) {}
    pub fn hash(&self, key: &[Transition]) -> usize {}
    pub fn get(&mut self, key: &[Transition], hash: usize) -> Option<StateID> {}
    pub fn set(&mut self, key: Vec<Transition>, hash: usize, state_id: StateID) {
        self.map[hash] = Utf8BoundedEntry {
            version: self.version,
            key,
            val: state_id,
        };
    }
}
