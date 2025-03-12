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
    pub fn new(capacity: usize) -> Utf8SuffixMap {
        assert!(capacity > 0);
        Utf8SuffixMap {
            version: 0,
            capacity,
            map: vec![],
        }
    }
    pub fn clear(&mut self) {}
    pub fn hash(&self, key: &Utf8SuffixKey) -> usize {}
    pub fn get(&mut self, key: &Utf8SuffixKey, hash: usize) -> Option<StateID> {}
    pub fn set(&mut self, key: Utf8SuffixKey, hash: usize, state_id: StateID) {}
}
