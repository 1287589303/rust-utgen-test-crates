#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "dfa-build")]
use crate::dfa::dense::BuildError;
use crate::{
    dfa::{automaton::Automaton, dense},
    util::{iter, search::Input},
    Anchored, Match, MatchError,
};
#[cfg(feature = "alloc")]
use crate::{
    dfa::{sparse, StartKind},
    util::search::MatchKind,
};
#[derive(Clone, Debug)]
pub struct Builder {
    #[cfg(feature = "dfa-build")]
    dfa: dense::Builder,
}
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Clone, Copy, Debug)]
pub struct Config {
    case_insensitive: bool,
    multi_line: bool,
    dot_matches_new_line: bool,
    crlf: bool,
    line_terminator: u8,
    swap_greed: bool,
    ignore_whitespace: bool,
    unicode: bool,
    utf8: bool,
    nest_limit: u32,
    octal: bool,
}
#[derive(Clone, Debug)]
pub struct Builder {
    dfa: dfa::Builder,
}
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    ast: ast::parse::ParserBuilder,
    hir: hir::translate::TranslatorBuilder,
}
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Clone, Debug, Default)]
pub struct Builder {
    /// The ID of the pattern that we're currently building.
    ///
    /// Callers are required to set (and unset) this by calling
    /// {start,finish}_pattern. Otherwise, most methods will panic.
    pattern_id: Option<PatternID>,
    /// A sequence of intermediate NFA states. Once a state is added to this
    /// sequence, it is assigned a state ID equivalent to its index. Once a
    /// state is added, it is still expected to be mutated, e.g., to set its
    /// transition to a state that didn't exist at the time it was added.
    states: Vec<State>,
    /// The starting states for each individual pattern. Starting at any
    /// of these states will result in only an anchored search for the
    /// corresponding pattern. The vec is indexed by pattern ID. When the NFA
    /// contains a single regex, then `start_pattern[0]` and `start_anchored`
    /// are always equivalent.
    start_pattern: Vec<StateID>,
    /// A map from pattern ID to capture group index to name. (If no name
    /// exists, then a None entry is present. Thus, all capturing groups are
    /// present in this mapping.)
    ///
    /// The outer vec is indexed by pattern ID, while the inner vec is indexed
    /// by capture index offset for the corresponding pattern.
    ///
    /// The first capture group for each pattern is always unnamed and is thus
    /// always None.
    captures: Vec<Vec<Option<Arc<str>>>>,
    /// The combined memory used by each of the 'State's in 'states'. This
    /// only includes heap usage by each state, and not the size of the state
    /// itself. In other words, this tracks heap memory used that isn't
    /// captured via `size_of::<State>() * states.len()`.
    memory_states: usize,
    /// Whether this NFA only matches UTF-8 and whether regex engines using
    /// this NFA for searching should report empty matches that split a
    /// codepoint.
    utf8: bool,
    /// Whether this NFA should be matched in reverse or not.
    reverse: bool,
    /// The matcher to use for look-around assertions.
    look_matcher: LookMatcher,
    /// A size limit to respect when building an NFA. If the total heap memory
    /// of the intermediate NFA states exceeds (or would exceed) this amount,
    /// then an error is returned.
    size_limit: Option<usize>,
}
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
impl Builder {
    pub fn new() -> Builder {}
    #[cfg(all(feature = "syntax", feature = "dfa-build"))]
    pub fn build(&self, pattern: &str) -> Result<Regex, BuildError> {}
    #[cfg(all(feature = "syntax", feature = "dfa-build"))]
    pub fn build_sparse(
        &self,
        pattern: &str,
    ) -> Result<Regex<sparse::DFA<Vec<u8>>>, BuildError> {}
    #[cfg(all(feature = "syntax", feature = "dfa-build"))]
    pub fn build_many<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<Regex, BuildError> {}
    #[cfg(all(feature = "syntax", feature = "dfa-build"))]
    pub fn build_many_sparse<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<Regex<sparse::DFA<Vec<u8>>>, BuildError> {}
    pub fn build_from_dfas<A: Automaton>(&self, forward: A, reverse: A) -> Regex<A> {}
    #[cfg(all(feature = "syntax", feature = "dfa-build"))]
    pub fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder {
        self.dfa.syntax(config);
        self
    }
    #[cfg(all(feature = "syntax", feature = "dfa-build"))]
    pub fn thompson(&mut self, config: crate::nfa::thompson::Config) -> &mut Builder {}
    #[cfg(feature = "dfa-build")]
    pub fn dense(&mut self, config: dense::Config) -> &mut Builder {}
}
#[cfg(feature = "dfa-build")]
impl Builder {
    pub fn new() -> Builder {}
    #[cfg(feature = "syntax")]
    pub fn build(&self, pattern: &str) -> Result<OwnedDFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn build_many<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<OwnedDFA, BuildError> {}
    pub fn build_from_nfa(&self, nfa: &thompson::NFA) -> Result<OwnedDFA, BuildError> {}
    pub fn configure(&mut self, config: Config) -> &mut Builder {}
    #[cfg(feature = "syntax")]
    pub fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder {
        self.thompson.syntax(config);
        self
    }
    #[cfg(feature = "syntax")]
    pub fn thompson(&mut self, config: thompson::Config) -> &mut Builder {}
}
