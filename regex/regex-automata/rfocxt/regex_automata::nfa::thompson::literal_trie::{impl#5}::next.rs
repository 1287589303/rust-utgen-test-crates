use core::mem;
use alloc::{vec, vec::Vec};
use crate::{
    nfa::thompson::{self, compiler::ThompsonRef, BuildError, Builder},
    util::primitives::{IteratorIndexExt, StateID},
};
#[derive(Debug)]
struct StateChunksIter<'a> {
    transitions: &'a [Transition],
    chunks: core::slice::Iter<'a, (usize, usize)>,
    active: Option<&'a [Transition]>,
}
#[derive(Clone, Copy, Eq, PartialEq)]
struct Transition(u64);
#[derive(Clone, Copy)]
struct Transition {
    byte: u8,
    next: StateID,
}
#[derive(Clone)]
struct Transition {
    /// The byte range.
    range: Utf8Range,
    /// The next state to transition to.
    next_id: StateID,
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
impl<'a> Iterator for StateChunksIter<'a> {
    type Item = &'a [Transition];
    fn next(&mut self) -> Option<&'a [Transition]> {
        if let Some(&(start, end)) = self.chunks.next() {
            return Some(&self.transitions[start..end]);
        }
        if let Some(chunk) = self.active.take() {
            return Some(chunk);
        }
        None
    }
}
