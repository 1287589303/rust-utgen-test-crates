#[cfg(feature = "alloc")]
pub(crate) type OwnedDFA = DFA<alloc::vec::Vec<u32>>;
#[cfg(feature = "dfa-build")]
use core::cmp;
use core::{fmt, iter, mem::size_of, slice};
#[cfg(feature = "dfa-build")]
use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec, vec::Vec,
};
#[cfg(feature = "dfa-build")]
use crate::{
    dfa::{accel::Accel, determinize, minimize::Minimizer, remapper::Remapper, sparse},
    nfa::thompson, util::{look::LookMatcher, search::MatchKind},
};
use crate::{
    dfa::{
        accel::Accels, automaton::{fmt_state_indicator, Automaton, StartError},
        special::Special, start::StartKind, DEAD,
    },
    util::{
        alphabet::{self, ByteClasses, ByteSet},
        int::{Pointer, Usize},
        prefilter::Prefilter, primitives::{PatternID, StateID},
        search::Anchored, start::{self, Start, StartByteMap},
        wire::{self, DeserializeError, Endian, SerializeError},
    },
};
const LABEL: &str = "rust-regex-automata-dfa-dense";
const VERSION: u32 = 2;
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug, Default)]
pub struct Config {
    accelerate: Option<bool>,
    pre: Option<Option<Prefilter>>,
    minimize: Option<bool>,
    match_kind: Option<MatchKind>,
    start_kind: Option<StartKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    unicode_word_boundary: Option<bool>,
    quitset: Option<ByteSet>,
    specialize_start_states: Option<bool>,
    dfa_size_limit: Option<Option<usize>>,
    determinize_size_limit: Option<Option<usize>>,
}
#[derive(Clone, Debug)]
pub struct Prefilter {
    #[cfg(not(feature = "alloc"))]
    _unused: (),
    #[cfg(feature = "alloc")]
    pre: Arc<dyn PrefilterI>,
    #[cfg(feature = "alloc")]
    is_fast: bool,
    #[cfg(feature = "alloc")]
    max_needle_len: usize,
}
#[derive(Clone, Debug)]
pub(crate) struct ByteSet([bool; 256]);
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
}
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StartKind {
    /// Support both anchored and unanchored searches.
    Both,
    /// Support only unanchored searches. Requesting an anchored search will
    /// panic.
    ///
    /// Note that even if an unanchored search is requested, the pattern itself
    /// may still be anchored. For example, `^abc` will only match `abc` at the
    /// start of a haystack. This will remain true, even if the regex engine
    /// only supported unanchored searches.
    Unanchored,
    /// Support only anchored searches. Requesting an unanchored search will
    /// panic.
    Anchored,
}
#[cfg(feature = "dfa-build")]
impl Config {
    pub fn new() -> Config {}
    pub fn accelerate(mut self, yes: bool) -> Config {}
    pub fn prefilter(mut self, pre: Option<Prefilter>) -> Config {}
    pub fn minimize(mut self, yes: bool) -> Config {}
    pub fn match_kind(mut self, kind: MatchKind) -> Config {}
    pub fn start_kind(mut self, kind: StartKind) -> Config {}
    pub fn starts_for_each_pattern(mut self, yes: bool) -> Config {}
    pub fn byte_classes(mut self, yes: bool) -> Config {}
    pub fn unicode_word_boundary(mut self, yes: bool) -> Config {}
    pub fn quit(mut self, byte: u8, yes: bool) -> Config {}
    pub fn specialize_start_states(mut self, yes: bool) -> Config {}
    pub fn dfa_size_limit(mut self, bytes: Option<usize>) -> Config {}
    pub fn determinize_size_limit(mut self, bytes: Option<usize>) -> Config {}
    pub fn get_accelerate(&self) -> bool {}
    pub fn get_prefilter(&self) -> Option<&Prefilter> {}
    pub fn get_minimize(&self) -> bool {}
    pub fn get_match_kind(&self) -> MatchKind {}
    pub fn get_starts(&self) -> StartKind {}
    pub fn get_starts_for_each_pattern(&self) -> bool {}
    pub fn get_byte_classes(&self) -> bool {}
    pub fn get_unicode_word_boundary(&self) -> bool {}
    pub fn get_quit(&self, byte: u8) -> bool {}
    pub fn get_specialize_start_states(&self) -> bool {}
    pub fn get_dfa_size_limit(&self) -> Option<usize> {}
    pub fn get_determinize_size_limit(&self) -> Option<usize> {
        self.determinize_size_limit.unwrap_or(None)
    }
    pub(crate) fn overwrite(&self, o: Config) -> Config {}
}
