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
#[derive(Clone, Debug, Default)]
pub struct Config {
    utf8: Option<bool>,
    reverse: Option<bool>,
    nfa_size_limit: Option<Option<usize>>,
    shrink: Option<bool>,
    which_captures: Option<WhichCaptures>,
    look_matcher: Option<LookMatcher>,
    #[cfg(test)]
    unanchored_prefix: Option<bool>,
}
#[derive(Clone, Debug)]
pub struct LookMatcher {
    lineterm: DebugByte,
}
#[derive(Clone, Copy, Debug)]
pub enum WhichCaptures {
    /// All capture states, including those corresponding to both implicit and
    /// explicit capture groups, are included in the Thompson NFA.
    All,
    /// Only capture states corresponding to implicit capture groups are
    /// included. Implicit capture groups appear in every pattern implicitly
    /// and correspond to the overall match of a pattern.
    ///
    /// This is useful when one only cares about the overall match of a
    /// pattern. By excluding capture states from explicit capture groups,
    /// one might be able to reduce the memory usage of a multi-pattern regex
    /// substantially if it was otherwise written to have many explicit capture
    /// groups.
    Implicit,
    /// No capture states are compiled into the Thompson NFA.
    ///
    /// This is useful when capture states are either not needed (for example,
    /// if one is only trying to build a DFA) or if they aren't supported (for
    /// example, a reverse NFA).
    None,
}
impl Config {
    pub fn new() -> Config {}
    pub fn utf8(mut self, yes: bool) -> Config {}
    pub fn reverse(mut self, yes: bool) -> Config {}
    pub fn nfa_size_limit(mut self, bytes: Option<usize>) -> Config {}
    pub fn shrink(mut self, yes: bool) -> Config {}
    #[deprecated(since = "0.3.5", note = "use which_captures instead")]
    pub fn captures(self, yes: bool) -> Config {
        self.which_captures(if yes { WhichCaptures::All } else { WhichCaptures::None })
    }
    pub fn which_captures(mut self, which_captures: WhichCaptures) -> Config {
        self.which_captures = Some(which_captures);
        self
    }
    pub fn look_matcher(mut self, m: LookMatcher) -> Config {}
    #[cfg(test)]
    fn unanchored_prefix(mut self, yes: bool) -> Config {}
    pub fn get_utf8(&self) -> bool {}
    pub fn get_reverse(&self) -> bool {}
    pub fn get_nfa_size_limit(&self) -> Option<usize> {}
    pub fn get_shrink(&self) -> bool {}
    #[deprecated(since = "0.3.5", note = "use get_which_captures instead")]
    pub fn get_captures(&self) -> bool {}
    pub fn get_which_captures(&self) -> WhichCaptures {}
    pub fn get_look_matcher(&self) -> LookMatcher {}
    fn get_unanchored_prefix(&self) -> bool {}
    pub(crate) fn overwrite(&self, o: Config) -> Config {}
}
