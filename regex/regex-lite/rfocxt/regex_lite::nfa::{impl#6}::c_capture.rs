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
pub(crate) trait U32 {
    fn as_usize(self) -> usize;
}
#[derive(Debug)]
struct Compiler {
    config: Config,
    nfa: RefCell<NFA>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    msg: &'static str,
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
    pub(crate) size_limit: Option<usize>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Hir {
    kind: HirKind,
    is_start_anchored: bool,
    is_match_empty: bool,
    static_explicit_captures_len: Option<usize>,
}
#[derive(Clone, Copy, Debug)]
struct ThompsonRef {
    start: StateID,
    end: StateID,
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
#[derive(Clone, Eq, PartialEq)]
pub(crate) enum State {
    Char { target: StateID, ch: char },
    Ranges { target: StateID, ranges: Vec<(char, char)> },
    Splits { targets: Vec<StateID>, reverse: bool },
    Goto { target: StateID, look: Option<hir::Look> },
    Capture { target: StateID, slot: u32 },
    Fail,
    Match,
}
impl Compiler {
    fn new(config: Config, pattern: String) -> Compiler {}
    fn compile(self, hir: &Hir) -> Result<NFA, Error> {}
    fn c(&self, hir: &Hir) -> Result<ThompsonRef, Error> {
        match *hir.kind() {
            HirKind::Empty => self.c_empty(),
            HirKind::Char(ch) => self.c_char(ch),
            HirKind::Class(ref class) => self.c_class(class),
            HirKind::Look(ref look) => self.c_look(look),
            HirKind::Repetition(ref rep) => self.c_repetition(rep),
            HirKind::Capture(ref cap) => {
                self.c_capture(cap.index, cap.name.as_deref(), &cap.sub)
            }
            HirKind::Concat(ref subs) => self.c_concat(subs.iter().map(|s| self.c(s))),
            HirKind::Alternation(ref subs) => {
                self.c_alternation(subs.iter().map(|s| self.c(s)))
            }
        }
    }
    fn c_fail(&self) -> Result<ThompsonRef, Error> {}
    fn c_empty(&self) -> Result<ThompsonRef, Error> {}
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
    ) -> Result<ThompsonRef, Error> {
        let existing_groups_len = self.nfa.borrow().cap_index_to_name.len();
        for _ in 0..(index.as_usize().saturating_sub(existing_groups_len)) {
            self.nfa.borrow_mut().cap_index_to_name.push(None);
        }
        if index.as_usize() >= existing_groups_len {
            if let Some(name) = name {
                let name = Arc::from(name);
                let mut nfa = self.nfa.borrow_mut();
                nfa.cap_name_to_index.insert(Arc::clone(&name), index);
                nfa.cap_index_to_name.push(Some(Arc::clone(&name)));
                nfa.memory_extra += name.len() + size_of::<u32>();
            } else {
                self.nfa.borrow_mut().cap_index_to_name.push(None);
            }
        }
        let Some(slot) = index.checked_mul(2) else {
            return Err(Error::new("capture group slots exhausted"));
        };
        let start = self.add(State::Capture { target: 0, slot })?;
        let inner = self.c(hir)?;
        let Some(slot) = slot.checked_add(1) else {
            return Err(Error::new("capture group slots exhausted"));
        };
        let end = self.add(State::Capture { target: 0, slot })?;
        self.patch(start, inner.start)?;
        self.patch(inner.end, end)?;
        Ok(ThompsonRef { start, end })
    }
    fn c_concat<I>(&self, mut it: I) -> Result<ThompsonRef, Error>
    where
        I: Iterator<Item = Result<ThompsonRef, Error>>,
    {}
    fn c_alternation<I>(&self, mut it: I) -> Result<ThompsonRef, Error>
    where
        I: Iterator<Item = Result<ThompsonRef, Error>>,
    {}
    fn add_empty(&self) -> Result<StateID, Error> {}
    fn add(&self, state: State) -> Result<StateID, Error> {
        let id = u32::try_from(self.nfa.borrow().states.len())
            .map_err(|_| Error::new("exhausted state IDs, too many states"))?;
        self.nfa.borrow_mut().memory_extra += state.memory_usage();
        self.nfa.borrow_mut().states.push(state);
        self.check_size_limit()?;
        Ok(id)
    }
    fn patch(&self, from: StateID, to: StateID) -> Result<(), Error> {
        let mut new_memory_extra = self.nfa.borrow().memory_extra;
        match self.nfa.borrow_mut().states[from.as_usize()] {
            State::Char { ref mut target, .. } => {
                *target = to;
            }
            State::Ranges { ref mut target, .. } => {
                *target = to;
            }
            State::Splits { ref mut targets, .. } => {
                targets.push(to);
                new_memory_extra += size_of::<StateID>();
            }
            State::Goto { ref mut target, .. } => {
                *target = to;
            }
            State::Capture { ref mut target, .. } => {
                *target = to;
            }
            State::Fail | State::Match => {}
        }
        if new_memory_extra != self.nfa.borrow().memory_extra {
            self.nfa.borrow_mut().memory_extra = new_memory_extra;
            self.check_size_limit()?;
        }
        Ok(())
    }
    fn check_size_limit(&self) -> Result<(), Error> {}
}
impl Error {
    pub(crate) fn new(msg: &'static str) -> Error {
        Error { msg }
    }
}
