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
#[derive(Clone, Copy, Eq, PartialEq)]
struct Transition(u64);
#[derive(Clone, Copy)]
struct Transition {
    byte: u8,
    next: StateID,
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
impl State {
    fn add_match(&mut self) {
        if self.transitions.is_empty() && !self.chunks.is_empty() {
            return;
        }
        let chunk_start = self.active_chunk_start();
        let chunk_end = self.transitions.len();
        self.chunks.push((chunk_start, chunk_end));
    }
    fn is_leaf(&self) -> bool {}
    fn chunks(&self) -> StateChunksIter<'_> {}
    fn active_chunk(&self) -> &[Transition] {}
    fn active_chunk_start(&self) -> usize {
        self.chunks.last().map_or(0, |&(_, end)| end)
    }
}
