use core::mem;
use alloc::{vec, vec::Vec};
use crate::{
    nfa::thompson::{self, compiler::ThompsonRef, BuildError, Builder},
    util::primitives::{IteratorIndexExt, StateID},
};
#[derive(Debug)]
struct Frame<'a> {
    /// The remaining chunks to visit for a trie state.
    chunks: StateChunksIter<'a>,
    /// The transitions of the current chunk that we're iterating over. Since
    /// every trie state has at least one chunk, every frame is initialized
    /// with the first chunk's transitions ready to be consumed.
    transitions: core::slice::Iter<'a, Transition>,
    /// The NFA state IDs pointing to the start of each chunk compiled by
    /// this trie state. This ultimately gets converted to an NFA union once
    /// the entire trie state (and all of its children) have been compiled.
    /// The order of these matters for leftmost-first match semantics, since
    /// earlier matches in the union are preferred over later ones.
    union: Vec<StateID>,
    /// The actual NFA transitions for a single chunk in a trie state. This
    /// gets converted to an NFA sparse state, and its corresponding NFA state
    /// ID should get added to 'union'.
    sparse: Vec<thompson::Transition>,
}
#[derive(Clone, Default)]
struct State {
    transitions: Vec<Transition>,
    chunks: Vec<(usize, usize)>,
}
#[derive(Clone, Copy)]
struct Transition {
    byte: u8,
    next: StateID,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Transition {
    /// The inclusive start of the byte range.
    pub start: u8,
    /// The inclusive end of the byte range.
    pub end: u8,
    /// The identifier of the state to transition to.
    pub next: StateID,
}
#[derive(Clone)]
struct Transition {
    /// The byte range.
    range: Utf8Range,
    /// The next state to transition to.
    next_id: StateID,
}
#[derive(Debug)]
struct StateChunksIter<'a> {
    transitions: &'a [Transition],
    chunks: core::slice::Iter<'a, (usize, usize)>,
    active: Option<&'a [Transition]>,
}
#[derive(Clone, Copy, Eq, PartialEq)]
struct Transition(u64);
impl<'a> Frame<'a> {
    fn new(state: &'a State) -> Frame<'a> {
        let mut chunks = state.chunks();
        let chunk = chunks.next().unwrap();
        let transitions = chunk.iter();
        Frame {
            chunks,
            transitions,
            union: vec![],
            sparse: vec![],
        }
    }
}
impl State {
    fn add_match(&mut self) {}
    fn is_leaf(&self) -> bool {}
    fn chunks(&self) -> StateChunksIter<'_> {
        StateChunksIter {
            transitions: &*self.transitions,
            chunks: self.chunks.iter(),
            active: Some(self.active_chunk()),
        }
    }
    fn active_chunk(&self) -> &[Transition] {}
    fn active_chunk_start(&self) -> usize {}
}
