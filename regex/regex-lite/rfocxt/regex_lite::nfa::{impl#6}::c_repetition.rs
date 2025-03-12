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
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Hir {
    kind: HirKind,
    is_start_anchored: bool,
    is_match_empty: bool,
    static_explicit_captures_len: Option<usize>,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    msg: &'static str,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct Config {
    pub(crate) size_limit: Option<usize>,
}
impl Compiler {
    fn new(config: Config, pattern: String) -> Compiler {}
    fn compile(self, hir: &Hir) -> Result<NFA, Error> {}
    fn c(&self, hir: &Hir) -> Result<ThompsonRef, Error> {}
    fn c_fail(&self) -> Result<ThompsonRef, Error> {}
    fn c_empty(&self) -> Result<ThompsonRef, Error> {}
    fn c_char(&self, ch: char) -> Result<ThompsonRef, Error> {}
    fn c_class(&self, class: &hir::Class) -> Result<ThompsonRef, Error> {}
    fn c_look(&self, look: &hir::Look) -> Result<ThompsonRef, Error> {}
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
    ) -> Result<ThompsonRef, Error> {
        let prefix = self.c_exactly(hir, min)?;
        if min == max {
            return Ok(prefix);
        }
        let empty = self.add_empty()?;
        let mut prev_end = prefix.end;
        for _ in min..max {
            let splits = self
                .add(State::Splits {
                    targets: vec![],
                    reverse: !greedy,
                })?;
            let compiled = self.c(hir)?;
            self.patch(prev_end, splits)?;
            self.patch(splits, compiled.start)?;
            self.patch(splits, empty)?;
            prev_end = compiled.end;
        }
        self.patch(prev_end, empty)?;
        Ok(ThompsonRef {
            start: prefix.start,
            end: empty,
        })
    }
    fn c_at_least(&self, hir: &Hir, greedy: bool, n: u32) -> Result<ThompsonRef, Error> {
        if n == 0 {
            if !hir.is_match_empty() {
                let splits = self
                    .add(State::Splits {
                        targets: vec![],
                        reverse: !greedy,
                    })?;
                let compiled = self.c(hir)?;
                self.patch(splits, compiled.start)?;
                self.patch(compiled.end, splits)?;
                return Ok(ThompsonRef {
                    start: splits,
                    end: splits,
                });
            }
            let compiled = self.c(hir)?;
            let plus = self
                .add(State::Splits {
                    targets: vec![],
                    reverse: !greedy,
                })?;
            self.patch(compiled.end, plus)?;
            self.patch(plus, compiled.start)?;
            let question = self
                .add(State::Splits {
                    targets: vec![],
                    reverse: !greedy,
                })?;
            let empty = self.add_empty()?;
            self.patch(question, compiled.start)?;
            self.patch(question, empty)?;
            self.patch(plus, empty)?;
            Ok(ThompsonRef {
                start: question,
                end: empty,
            })
        } else if n == 1 {
            let compiled = self.c(hir)?;
            let splits = self
                .add(State::Splits {
                    targets: vec![],
                    reverse: !greedy,
                })?;
            self.patch(compiled.end, splits)?;
            self.patch(splits, compiled.start)?;
            Ok(ThompsonRef {
                start: compiled.start,
                end: splits,
            })
        } else {
            let prefix = self.c_exactly(hir, n - 1)?;
            let last = self.c(hir)?;
            let splits = self
                .add(State::Splits {
                    targets: vec![],
                    reverse: !greedy,
                })?;
            self.patch(prefix.end, last.start)?;
            self.patch(last.end, splits)?;
            self.patch(splits, last.start)?;
            Ok(ThompsonRef {
                start: prefix.start,
                end: splits,
            })
        }
    }
    fn c_zero_or_one(&self, hir: &Hir, greedy: bool) -> Result<ThompsonRef, Error> {
        let splits = self
            .add(State::Splits {
                targets: vec![],
                reverse: !greedy,
            })?;
        let compiled = self.c(hir)?;
        let empty = self.add_empty()?;
        self.patch(splits, compiled.start)?;
        self.patch(splits, empty)?;
        self.patch(compiled.end, empty)?;
        Ok(ThompsonRef {
            start: splits,
            end: empty,
        })
    }
    fn c_exactly(&self, hir: &Hir, n: u32) -> Result<ThompsonRef, Error> {
        self.c_concat((0..n).map(|_| self.c(hir)))
    }
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
    fn add_empty(&self) -> Result<StateID, Error> {}
    fn add(&self, state: State) -> Result<StateID, Error> {}
    fn patch(&self, from: StateID, to: StateID) -> Result<(), Error> {}
    fn check_size_limit(&self) -> Result<(), Error> {}
}
