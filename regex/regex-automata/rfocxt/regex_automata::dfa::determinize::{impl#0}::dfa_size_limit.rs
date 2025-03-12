#[cfg(feature = "std")]
type StateMap = std::collections::HashMap<State, StateID>;
#[cfg(not(feature = "std"))]
type StateMap = BTreeMap<State, StateID>;
use alloc::{collections::BTreeMap, vec::Vec};
use crate::{
    dfa::{
        dense::{self, BuildError},
        DEAD,
    },
    nfa::thompson,
    util::{
        self, alphabet::{self, ByteSet},
        determinize::{State, StateBuilderEmpty, StateBuilderNFA},
        primitives::{PatternID, StateID},
        search::{Anchored, MatchKind},
        sparse_set::SparseSets, start::Start,
    },
};
#[derive(Clone, Debug)]
pub(crate) struct Config {
    match_kind: MatchKind,
    quit: ByteSet,
    dfa_size_limit: Option<usize>,
    determinize_size_limit: Option<usize>,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
}
#[derive(Clone, Debug)]
pub(crate) struct ByteSet([bool; 256]);
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchKind {
    /// Report all possible matches.
    All,
    /// Report only the leftmost matches. When multiple leftmost matches exist,
    /// report the match corresponding to the part of the regex that appears
    /// first in the syntax.
    LeftmostFirst,
}
impl Config {
    pub fn new() -> Config {}
    pub fn run(
        &self,
        nfa: &thompson::NFA,
        dfa: &mut dense::OwnedDFA,
    ) -> Result<(), BuildError> {}
    pub fn match_kind(&mut self, kind: MatchKind) -> &mut Config {}
    pub fn quit(&mut self, set: ByteSet) -> &mut Config {}
    pub fn dfa_size_limit(&mut self, bytes: Option<usize>) -> &mut Config {
        self.dfa_size_limit = bytes;
        self
    }
    pub fn determinize_size_limit(&mut self, bytes: Option<usize>) -> &mut Config {}
}
