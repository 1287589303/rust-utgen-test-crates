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
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Hir {
    kind: HirKind,
    is_start_anchored: bool,
    is_match_empty: bool,
    static_explicit_captures_len: Option<usize>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    msg: &'static str,
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
pub(crate) struct Capture {
    /// The capture index of the capture.
    pub(crate) index: u32,
    /// The name of the capture, if it exists.
    pub(crate) name: Option<Box<str>>,
    /// The expression inside the capturing group, which may be empty.
    pub(crate) sub: Box<Hir>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Repetition {
    /// The minimum range of the repetition.
    ///
    /// Note that special cases like `?`, `+` and `*` all get translated into
    /// the ranges `{0,1}`, `{1,}` and `{0,}`, respectively.
    ///
    /// When `min` is zero, this expression can match the empty string
    /// regardless of what its sub-expression is.
    pub(crate) min: u32,
    /// The maximum range of the repetition.
    ///
    /// Note that when `max` is `None`, `min` acts as a lower bound but where
    /// there is no upper bound. For something like `x{5}` where the min and
    /// max are equivalent, `min` will be set to `5` and `max` will be set to
    /// `Some(5)`.
    pub(crate) max: Option<u32>,
    /// Whether this repetition operator is greedy or not. A greedy operator
    /// will match as much as it can. A non-greedy operator will match as
    /// little as it can.
    ///
    /// Typically, operators are greedy by default and are only non-greedy when
    /// a `?` suffix is used, e.g., `(expr)*` is greedy while `(expr)*?` is
    /// not. However, this can be inverted via the `U` "ungreedy" flag.
    pub(crate) greedy: bool,
    /// The expression being repeated.
    pub(crate) sub: Box<Hir>,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Class {
    pub(crate) ranges: Vec<ClassRange>,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum HirKind {
    Empty,
    Char(char),
    Class(Class),
    Look(Look),
    Repetition(Repetition),
    Capture(Capture),
    Concat(Vec<Hir>),
    Alternation(Vec<Hir>),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Look {
    /// Match the beginning of text. Specifically, this matches at the starting
    /// position of the input.
    Start = 1 << 0,
    /// Match the end of text. Specifically, this matches at the ending
    /// position of the input.
    End = 1 << 1,
    /// Match the beginning of a line or the beginning of text. Specifically,
    /// this matches at the starting position of the input, or at the position
    /// immediately following a `\n` character.
    StartLF = 1 << 2,
    /// Match the end of a line or the end of text. Specifically, this matches
    /// at the end position of the input, or at the position immediately
    /// preceding a `\n` character.
    EndLF = 1 << 3,
    /// Match the beginning of a line or the beginning of text. Specifically,
    /// this matches at the starting position of the input, or at the position
    /// immediately following either a `\r` or `\n` character, but never after
    /// a `\r` when a `\n` follows.
    StartCRLF = 1 << 4,
    /// Match the end of a line or the end of text. Specifically, this matches
    /// at the end position of the input, or at the position immediately
    /// preceding a `\r` or `\n` character, but never before a `\n` when a `\r`
    /// precedes it.
    EndCRLF = 1 << 5,
    /// Match an ASCII-only word boundary. That is, this matches a position
    /// where the left adjacent character and right adjacent character
    /// correspond to a word and non-word or a non-word and word character.
    Word = 1 << 6,
    /// Match an ASCII-only negation of a word boundary.
    WordNegate = 1 << 7,
    /// Match the start of an ASCII-only word boundary. That is, this matches a
    /// position at either the beginning of the haystack or where the previous
    /// character is not a word character and the following character is a word
    /// character.
    WordStart = 1 << 8,
    /// Match the end of an ASCII-only word boundary. That is, this matches
    /// a position at either the end of the haystack or where the previous
    /// character is a word character and the following character is not a word
    /// character.
    WordEnd = 1 << 9,
    /// Match the start half of an ASCII-only word boundary. That is, this
    /// matches a position at either the beginning of the haystack or where the
    /// previous character is not a word character.
    WordStartHalf = 1 << 10,
    /// Match the end half of an ASCII-only word boundary. That is, this
    /// matches a position at either the end of the haystack or where the
    /// following character is not a word character.
    WordEndHalf = 1 << 11,
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
    fn c_empty(&self) -> Result<ThompsonRef, Error> {
        let id = self.add_empty()?;
        Ok(ThompsonRef { start: id, end: id })
    }
    fn c_char(&self, ch: char) -> Result<ThompsonRef, Error> {
        let id = self.add(State::Char { target: 0, ch })?;
        Ok(ThompsonRef { start: id, end: id })
    }
    fn c_class(&self, class: &hir::Class) -> Result<ThompsonRef, Error> {
        let id = if class.ranges.is_empty() {
            self.add(State::Fail)
        } else {
            let ranges = class.ranges.iter().map(|r| (r.start, r.end)).collect();
            self.add(State::Ranges { target: 0, ranges })
        }?;
        Ok(ThompsonRef { start: id, end: id })
    }
    fn c_look(&self, look: &hir::Look) -> Result<ThompsonRef, Error> {
        let id = self
            .add(State::Goto {
                target: 0,
                look: Some(*look),
            })?;
        Ok(ThompsonRef { start: id, end: id })
    }
    fn c_repetition(&self, rep: &hir::Repetition) -> Result<ThompsonRef, Error> {
        match (rep.min, rep.max) {
            (0, Some(1)) => self.c_zero_or_one(&rep.sub, rep.greedy),
            (min, None) => self.c_at_least(&rep.sub, rep.greedy, min),
            (min, Some(max)) if min == max => self.c_exactly(&rep.sub, min),
            (min, Some(max)) => self.c_bounded(&rep.sub, rep.greedy, min, max),
        }
    }
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
    fn add(&self, state: State) -> Result<StateID, Error> {}
    fn patch(&self, from: StateID, to: StateID) -> Result<(), Error> {}
    fn check_size_limit(&self) -> Result<(), Error> {}
}
impl Hir {
    pub(crate) fn parse(config: Config, pattern: &str) -> Result<Hir, Error> {}
    pub(crate) fn kind(&self) -> &HirKind {
        &self.kind
    }
    pub(crate) fn is_start_anchored(&self) -> bool {}
    pub(crate) fn is_match_empty(&self) -> bool {}
    pub(crate) fn static_explicit_captures_len(&self) -> Option<usize> {}
    fn fail() -> Hir {}
    fn empty() -> Hir {}
    fn char(ch: char) -> Hir {}
    fn class(class: Class) -> Hir {}
    fn look(look: Look) -> Hir {}
    fn repetition(rep: Repetition) -> Hir {}
    fn capture(cap: Capture) -> Hir {}
    fn concat(mut subs: Vec<Hir>) -> Hir {}
    fn alternation(mut subs: Vec<Hir>) -> Hir {}
}
