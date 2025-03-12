use core::mem;
use alloc::{vec, vec::Vec};
use crate::{
    nfa::thompson::{self, compiler::ThompsonRef, BuildError, Builder},
    util::primitives::{IteratorIndexExt, StateID},
};
#[derive(Clone, Default)]
struct State {
    transitions: Vec<Transition>,
    chunks: Vec<(usize, usize)>,
}
#[derive(Clone)]
struct Transition {
    /// The byte range.
    range: Utf8Range,
    /// The next state to transition to.
    next_id: StateID,
}
#[derive(Clone, Copy)]
struct Transition {
    byte: u8,
    next: StateID,
}
#[derive(Clone, Copy, Eq, PartialEq)]
struct Transition(u64);
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Transition {
    /// The inclusive start of the byte range.
    pub start: u8,
    /// The inclusive end of the byte range.
    pub end: u8,
    /// The identifier of the state to transition to.
    pub next: StateID,
}
#[derive(Debug)]
struct StateChunksIter<'a> {
    transitions: &'a [Transition],
    chunks: core::slice::Iter<'a, (usize, usize)>,
    active: Option<&'a [Transition]>,
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
    fn active_chunk(&self) -> &[Transition] {
        let start = self.active_chunk_start();
        &self.transitions[start..]
    }
    fn active_chunk_start(&self) -> usize {}
}
