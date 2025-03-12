use core::{borrow::Borrow, cell::RefCell};
use alloc::{sync::Arc, vec, vec::Vec};
use regex_syntax::{
    hir::{self, Hir},
    utf8::{Utf8Range, Utf8Sequences},
    ParserBuilder,
};
use crate::{
    nfa::thompson::{
        builder::Builder, error::BuildError, literal_trie::LiteralTrie,
        map::{Utf8BoundedMap, Utf8SuffixKey, Utf8SuffixMap},
        nfa::{Transition, NFA},
        range_trie::RangeTrie,
    },
    util::{
        look::{Look, LookMatcher},
        primitives::{PatternID, StateID},
    },
};
#[derive(Clone, Debug)]
struct Utf8State {
    compiled: Utf8BoundedMap,
    uncompiled: Vec<Utf8Node>,
}
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
#[derive(Clone, Debug)]
struct Utf8Node {
    trans: Vec<Transition>,
    last: Option<Utf8LastTransition>,
}
impl Utf8State {
    fn new() -> Utf8State {
        Utf8State {
            compiled: Utf8BoundedMap::new(10_000),
            uncompiled: vec![],
        }
    }
    fn clear(&mut self) {}
}
impl Utf8BoundedMap {
    pub fn new(capacity: usize) -> Utf8BoundedMap {
        assert!(capacity > 0);
        Utf8BoundedMap {
            version: 0,
            capacity,
            map: vec![],
        }
    }
    pub fn clear(&mut self) {}
    pub fn hash(&self, key: &[Transition]) -> usize {}
    pub fn get(&mut self, key: &[Transition], hash: usize) -> Option<StateID> {}
    pub fn set(&mut self, key: Vec<Transition>, hash: usize, state_id: StateID) {}
}
