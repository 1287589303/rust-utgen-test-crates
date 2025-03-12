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
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Literal {
    bytes: Vec<u8>,
    exact: bool,
}
#[derive(Debug, Default)]
struct State {
    /// Sparse representation of the transitions out of this state. Transitions
    /// are sorted by byte. There is at most one such transition for any
    /// particular byte.
    trans: Vec<(u8, usize)>,
}
impl PreferenceTrie {
    fn minimize(literals: &mut Vec<Literal>, keep_exact: bool) {
        let mut trie = PreferenceTrie {
            states: vec![],
            matches: vec![],
            next_literal_index: 1,
        };
        let mut make_inexact = vec![];
        literals
            .retain_mut(|lit| match trie.insert(lit.as_bytes()) {
                Ok(_) => true,
                Err(i) => {
                    if !keep_exact {
                        make_inexact.push(i.checked_sub(1).unwrap());
                    }
                    false
                }
            });
        for i in make_inexact {
            literals[i].make_inexact();
        }
    }
    fn insert(&mut self, bytes: &[u8]) -> Result<usize, usize> {}
    fn root(&mut self) -> usize {}
    fn create_state(&mut self) -> usize {}
}
impl Literal {
    #[inline]
    pub fn exact<B: Into<Vec<u8>>>(bytes: B) -> Literal {}
    #[inline]
    pub fn inexact<B: Into<Vec<u8>>>(bytes: B) -> Literal {}
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {}
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn is_exact(&self) -> bool {}
    #[inline]
    pub fn make_inexact(&mut self) {
        self.exact = false;
    }
    #[inline]
    pub fn reverse(&mut self) {}
    #[inline]
    pub fn extend(&mut self, lit: &Literal) {}
    #[inline]
    pub fn keep_first_bytes(&mut self, len: usize) {}
    #[inline]
    pub fn keep_last_bytes(&mut self, len: usize) {}
    fn is_poisonous(&self) -> bool {}
}
