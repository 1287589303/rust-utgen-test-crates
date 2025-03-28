pub(crate) type StateID = u32;
#[cfg(feature = "std")]
type CaptureNameMap = std::collections::HashMap<Arc<str>, u32>;
#[cfg(not(feature = "std"))]
type CaptureNameMap = alloc::collections::BTreeMap<Arc<str>, u32>;
use core::{cell::RefCell, mem::size_of};
use alloc::{string::String, sync::Arc, vec, vec::Vec};
use crate::{
    error::Error, hir::{self, Hir, HirKind},
    int::U32,
};
#[derive(Debug)]
struct Compiler {
    config: Config,
    nfa: RefCell<NFA>,
}
#[derive(Clone)]
pub(crate) struct NFA {
    /// The pattern string this NFA was generated from.
    ///
    /// We put it here for lack of a better place to put it. ¯\_(ツ)_/¯
    pattern: String,
    /// The states that make up this NFA.
    states: Vec<State>,
    /// The ID of the start state.
    start: StateID,
    /// Whether this NFA can only match at the beginning of a haystack.
    is_start_anchored: bool,
    /// Whether this NFA can match the empty string.
    is_match_empty: bool,
    /// If every match has the same number of matching capture groups, then
    /// this corresponds to the number of groups.
    static_explicit_captures_len: Option<usize>,
    /// A map from capture group name to its corresponding index.
    cap_name_to_index: CaptureNameMap,
    /// A map from capture group index to the corresponding name, if one
    /// exists.
    cap_index_to_name: Vec<Option<Arc<str>>>,
    /// Heap memory used indirectly by NFA states and other things (like the
    /// various capturing group representations above). Since each state
    /// might use a different amount of heap, we need to keep track of this
    /// incrementally.
    memory_extra: usize,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct Config {
    /// The maximum number of times we're allowed to recurse.
    ///
    /// Note that unlike the regex-syntax parser, we actually use recursion in
    /// this parser for simplicity. My hope is that by setting a conservative
    /// default call limit and providing a way to configure it, that we can
    /// keep this simplification. But if we must, we can re-work the parser to
    /// put the call stack on the heap like regex-syntax does.
    pub(crate) nest_limit: u32,
    /// Various flags that control how a pattern is interpreted.
    pub(crate) flags: Flags,
}
#[derive(Clone, Copy, Debug)]
struct ThompsonRef {
    start: StateID,
    end: StateID,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct Config {
    pub(crate) size_limit: Option<usize>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    msg: &'static str,
}
impl Compiler {
    fn new(config: Config, pattern: String) -> Compiler {}
    fn compile(self, hir: &Hir) -> Result<NFA, Error> {}
    fn c(&self, hir: &Hir) -> Result<ThompsonRef, Error> {}
    fn c_fail(&self) -> Result<ThompsonRef, Error> {}
    fn c_empty(&self) -> Result<ThompsonRef, Error> {
        let id = self.add_empty()?;
        Ok(ThompsonRef { start: id, end: id })
    }
    fn c_char(&self, ch: char) -> Result<ThompsonRef, Error> {}
    fn c_class(&self, class: &hir::Class) -> Result<ThompsonRef, Error> {}
    fn c_look(&self, look: &hir::Look) -> Result<ThompsonRef, Error> {}
    fn c_repetition(&self, rep: &hir::Repetition) -> Result<ThompsonRef, Error> {}
    fn c_bounded(
        &self,
        hir: &Hir,
        greedy: bool,
        min: u32,
        max: u32,
    ) -> Result<ThompsonRef, Error> {}
    fn c_at_least(&self, hir: &Hir, greedy: bool, n: u32) -> Result<ThompsonRef, Error> {}
    fn c_zero_or_one(&self, hir: &Hir, greedy: bool) -> Result<ThompsonRef, Error> {}
    fn c_exactly(&self, hir: &Hir, n: u32) -> Result<ThompsonRef, Error> {}
    fn c_capture(
        &self,
        index: u32,
        name: Option<&str>,
        hir: &Hir,
    ) -> Result<ThompsonRef, Error> {}
    fn c_concat<I>(&self, mut it: I) -> Result<ThompsonRef, Error>
    where
        I: Iterator<Item = Result<ThompsonRef, Error>>,
    {}
    fn c_alternation<I>(&self, mut it: I) -> Result<ThompsonRef, Error>
    where
        I: Iterator<Item = Result<ThompsonRef, Error>>,
    {}
    fn add_empty(&self) -> Result<StateID, Error> {
        self.add(State::Goto {
            target: 0,
            look: None,
        })
    }
    fn add(&self, state: State) -> Result<StateID, Error> {}
    fn patch(&self, from: StateID, to: StateID) -> Result<(), Error> {}
    fn check_size_limit(&self) -> Result<(), Error> {}
}
