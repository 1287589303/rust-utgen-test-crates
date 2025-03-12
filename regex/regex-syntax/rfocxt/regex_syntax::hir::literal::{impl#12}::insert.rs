use core::{cmp, mem, num::NonZeroUsize};
use alloc::{vec, vec::Vec};
use crate::hir::{self, Hir};
#[derive(Debug)]
struct PreferenceTrie {
    /// The states in this trie. The index of a state in this vector is its ID.
    states: Vec<State>,
    /// This vec indicates which states are match states. It always has
    /// the same length as `states` and is indexed by the same state ID.
    /// A state with identifier `sid` is a match state if and only if
    /// `matches[sid].is_some()`. The option contains the index of the literal
    /// corresponding to the match. The index is offset by 1 so that it fits in
    /// a NonZeroUsize.
    matches: Vec<Option<NonZeroUsize>>,
    /// The index to allocate to the next literal added to this trie. Starts at
    /// 1 and increments by 1 for every literal successfully added to the trie.
    next_literal_index: usize,
}
#[derive(Debug, Default)]
struct State {
    /// Sparse representation of the transitions out of this state. Transitions
    /// are sorted by byte. There is at most one such transition for any
    /// particular byte.
    trans: Vec<(u8, usize)>,
}
impl PreferenceTrie {
    fn minimize(literals: &mut Vec<Literal>, keep_exact: bool) {}
    fn insert(&mut self, bytes: &[u8]) -> Result<usize, usize> {
        let mut prev = self.root();
        if let Some(idx) = self.matches[prev] {
            return Err(idx.get());
        }
        for &b in bytes.iter() {
            match self.states[prev].trans.binary_search_by_key(&b, |t| t.0) {
                Ok(i) => {
                    prev = self.states[prev].trans[i].1;
                    if let Some(idx) = self.matches[prev] {
                        return Err(idx.get());
                    }
                }
                Err(i) => {
                    let next = self.create_state();
                    self.states[prev].trans.insert(i, (b, next));
                    prev = next;
                }
            }
        }
        let idx = self.next_literal_index;
        self.next_literal_index += 1;
        self.matches[prev] = NonZeroUsize::new(idx);
        Ok(idx)
    }
    fn root(&mut self) -> usize {
        if !self.states.is_empty() { 0 } else { self.create_state() }
    }
    fn create_state(&mut self) -> usize {
        let id = self.states.len();
        self.states.push(State::default());
        self.matches.push(None);
        id
    }
}
