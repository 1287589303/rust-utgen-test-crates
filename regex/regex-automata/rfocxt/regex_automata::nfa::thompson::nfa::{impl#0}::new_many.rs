use core::{fmt, mem};
use alloc::{boxed::Box, format, string::String, sync::Arc, vec, vec::Vec};
#[cfg(feature = "syntax")]
use crate::nfa::thompson::{
    compiler::{Compiler, Config},
    error::BuildError,
};
use crate::{
    nfa::thompson::builder::Builder,
    util::{
        alphabet::{self, ByteClassSet, ByteClasses},
        captures::{GroupInfo, GroupInfoError},
        look::{Look, LookMatcher, LookSet},
        primitives::{IteratorIndexExt, PatternID, PatternIDIter, SmallIndex, StateID},
        sparse_set::SparseSet,
    },
};
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Debug)]
pub struct Compiler {
    /// A regex parser, used when compiling an NFA directly from a pattern
    /// string.
    parser: ParserBuilder,
    /// The compiler configuration.
    config: Config,
    /// The builder for actually constructing an NFA. This provides a
    /// convenient abstraction for writing a compiler.
    builder: RefCell<Builder>,
    /// State used for compiling character classes to UTF-8 byte automata.
    /// State is not retained between character class compilations. This just
    /// serves to amortize allocation to the extent possible.
    utf8_state: RefCell<Utf8State>,
    /// State used for arranging character classes in reverse into a trie.
    trie_state: RefCell<RangeTrie>,
    /// State used for caching common suffixes when compiling reverse UTF-8
    /// automata (for Unicode character classes).
    utf8_suffix: RefCell<Utf8SuffixMap>,
}
#[derive(Default)]
pub(super) struct Inner {
    /// The state sequence. This sequence is guaranteed to be indexable by all
    /// starting state IDs, and it is also guaranteed to contain at most one
    /// `Match` state for each pattern compiled into this NFA. (A pattern may
    /// not have a corresponding `Match` state if a `Match` state is impossible
    /// to reach.)
    states: Vec<State>,
    /// The anchored starting state of this NFA.
    start_anchored: StateID,
    /// The unanchored starting state of this NFA.
    start_unanchored: StateID,
    /// The starting states for each individual pattern. Starting at any
    /// of these states will result in only an anchored search for the
    /// corresponding pattern. The vec is indexed by pattern ID. When the NFA
    /// contains a single regex, then `start_pattern[0]` and `start_anchored`
    /// are always equivalent.
    start_pattern: Vec<StateID>,
    /// Info about the capturing groups in this NFA. This is responsible for
    /// mapping groups to slots, mapping groups to names and names to groups.
    group_info: GroupInfo,
    /// A representation of equivalence classes over the transitions in this
    /// NFA. Two bytes in the same equivalence class must not discriminate
    /// between a match or a non-match. This map can be used to shrink the
    /// total size of a DFA's transition table with a small match-time cost.
    ///
    /// Note that the NFA's transitions are *not* defined in terms of these
    /// equivalence classes. The NFA's transitions are defined on the original
    /// byte values. For the most part, this is because they wouldn't really
    /// help the NFA much since the NFA already uses a sparse representation
    /// to represent transitions. Byte classes are most effective in a dense
    /// representation.
    byte_class_set: ByteClassSet,
    /// This is generated from `byte_class_set`, and essentially represents the
    /// same thing but supports different access patterns. Namely, this permits
    /// looking up the equivalence class of a byte very cheaply.
    ///
    /// Ideally we would just store this, but because of annoying code
    /// structure reasons, we keep both this and `byte_class_set` around for
    /// now. I think I would prefer that `byte_class_set` were computed in the
    /// `Builder`, but right now, we compute it as states are added to the
    /// `NFA`.
    byte_classes: ByteClasses,
    /// Whether this NFA has a `Capture` state anywhere.
    has_capture: bool,
    /// When the empty string is in the language matched by this NFA.
    has_empty: bool,
    /// Whether UTF-8 mode is enabled for this NFA. Briefly, this means that
    /// all non-empty matches produced by this NFA correspond to spans of valid
    /// UTF-8, and any empty matches produced by this NFA that split a UTF-8
    /// encoded codepoint should be filtered out by the corresponding regex
    /// engine.
    utf8: bool,
    /// Whether this NFA is meant to be matched in reverse or not.
    reverse: bool,
    /// The matcher to be used for look-around assertions.
    look_matcher: LookMatcher,
    /// The union of all look-around assertions that occur anywhere within
    /// this NFA. If this set is empty, then it means there are precisely zero
    /// conditional epsilon transitions in the NFA.
    look_set_any: LookSet,
    /// The union of all look-around assertions that occur as a zero-length
    /// prefix for any of the patterns in this NFA.
    look_set_prefix_any: LookSet,
    /// Heap memory used indirectly by NFA states and other things (like the
    /// various capturing group representations above). Since each state
    /// might use a different amount of heap, we need to keep track of this
    /// incrementally.
    memory_extra: usize,
}
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
impl NFA {
    #[cfg(feature = "syntax")]
    pub fn new(pattern: &str) -> Result<NFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<NFA, BuildError> {
        NFA::compiler().build_many(patterns)
    }
    pub fn always_match() -> NFA {}
    pub fn never_match() -> NFA {}
    #[cfg(feature = "syntax")]
    pub fn config() -> Config {}
    #[cfg(feature = "syntax")]
    pub fn compiler() -> Compiler {
        Compiler::new()
    }
    pub fn patterns(&self) -> PatternIter<'_> {}
    #[inline]
    pub fn pattern_len(&self) -> usize {}
    #[inline]
    pub fn start_anchored(&self) -> StateID {}
    #[inline]
    pub fn start_unanchored(&self) -> StateID {}
    #[inline]
    pub fn start_pattern(&self, pid: PatternID) -> Option<StateID> {}
    #[inline]
    pub(crate) fn byte_class_set(&self) -> &ByteClassSet {}
    #[inline]
    pub fn byte_classes(&self) -> &ByteClasses {}
    #[inline]
    pub fn state(&self, id: StateID) -> &State {}
    #[inline]
    pub fn states(&self) -> &[State] {}
    #[inline]
    pub fn group_info(&self) -> &GroupInfo {}
    #[inline]
    pub fn has_capture(&self) -> bool {}
    #[inline]
    pub fn has_empty(&self) -> bool {}
    #[inline]
    pub fn is_utf8(&self) -> bool {}
    #[inline]
    pub fn is_reverse(&self) -> bool {}
    #[inline]
    pub fn is_always_start_anchored(&self) -> bool {}
    #[inline]
    pub fn look_matcher(&self) -> &LookMatcher {}
    #[inline]
    pub fn look_set_any(&self) -> LookSet {}
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
impl Compiler {
    pub fn new() -> Compiler {}
    pub fn build(&self, pattern: &str) -> Result<NFA, BuildError> {}
    pub fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<NFA, BuildError> {
        let mut hirs = vec![];
        for p in patterns {
            hirs.push(
                self.parser.build().parse(p.as_ref()).map_err(BuildError::syntax)?,
            );
            debug!("parsed: {:?}", p.as_ref());
        }
        self.build_many_from_hir(&hirs)
    }
    pub fn build_from_hir(&self, expr: &Hir) -> Result<NFA, BuildError> {}
    pub fn build_many_from_hir<H: Borrow<Hir>>(
        &self,
        exprs: &[H],
    ) -> Result<NFA, BuildError> {}
    pub fn configure(&mut self, config: Config) -> &mut Compiler {}
    pub fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Compiler {}
}
