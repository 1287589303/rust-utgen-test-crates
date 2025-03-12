use alloc::vec::Vec;
pub(crate) use self::state::{
    State, StateBuilderEmpty, StateBuilderMatches, StateBuilderNFA,
};
use crate::{
    nfa::thompson,
    util::{
        alphabet, look::{Look, LookSet},
        primitives::StateID, search::MatchKind, sparse_set::{SparseSet, SparseSets},
        start::Start, utf8,
    },
};
#[derive(Clone, Debug)]
pub struct LookMatcher {
    lineterm: DebugByte,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone)]
pub(crate) struct StateBuilderMatches(Vec<u8>);
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct LookSet {
    /// The underlying representation this set is exposed to make it possible
    /// to store it somewhere efficiently. The representation is that
    /// of a bitset, where each assertion occupies bit `i` where
    /// `i = Look::as_repr()`.
    ///
    /// Note that users of this internal representation must permit the full
    /// range of `u16` values to be represented. For example, even if the
    /// current implementation only makes use of the 10 least significant bits,
    /// it may use more bits in a future semver compatible release.
    pub bits: u32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Start {
    /// This occurs when the starting position is not any of the ones below.
    NonWordByte = 0,
    /// This occurs when the byte immediately preceding the start of the search
    /// is an ASCII word byte.
    WordByte = 1,
    /// This occurs when the starting position of the search corresponds to the
    /// beginning of the haystack.
    Text = 2,
    /// This occurs when the byte immediately preceding the start of the search
    /// is a line terminator. Specifically, `\n`.
    LineLF = 3,
    /// This occurs when the byte immediately preceding the start of the search
    /// is a line terminator. Specifically, `\r`.
    LineCR = 4,
    /// This occurs when a custom line terminator has been set via a
    /// `LookMatcher`, and when that line terminator is neither a `\r` or a
    /// `\n`.
    ///
    /// If the custom line terminator is a word byte, then this start
    /// configuration is still selected. DFAs that implement word boundary
    /// assertions will likely need to check whether the custom line terminator
    /// is a word byte, in which case, it should behave as if the byte
    /// satisfies `\b` in addition to multi-line anchors.
    CustomLineTerminator = 5,
}
impl LookMatcher {
    pub fn new() -> LookMatcher {}
    pub fn set_line_terminator(&mut self, byte: u8) -> &mut LookMatcher {}
    pub fn get_line_terminator(&self) -> u8 {
        self.lineterm.0
    }
    #[inline]
    pub fn matches(&self, look: Look, haystack: &[u8], at: usize) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn matches_inline(&self, look: Look, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn matches_set(&self, set: LookSet, haystack: &[u8], at: usize) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn matches_set_inline(
        &self,
        set: LookSet,
        haystack: &[u8],
        at: usize,
    ) -> bool {}
    #[cfg(feature = "alloc")]
    pub(crate) fn add_to_byteset(
        &self,
        look: Look,
        set: &mut crate::util::alphabet::ByteClassSet,
    ) {}
    #[inline]
    pub fn is_start(&self, _haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_start_lf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end_lf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_start_crlf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end_crlf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_ascii_negate(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_unicode_negate(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_start_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_end_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_start_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_end_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_start_half_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_end_half_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_start_half_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_end_half_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
}
impl NFA {
    #[cfg(feature = "syntax")]
    pub fn new(pattern: &str) -> Result<NFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<NFA, BuildError> {}
    pub fn always_match() -> NFA {}
    pub fn never_match() -> NFA {}
    #[cfg(feature = "syntax")]
    pub fn config() -> Config {}
    #[cfg(feature = "syntax")]
    pub fn compiler() -> Compiler {}
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
    pub fn is_reverse(&self) -> bool {
        self.0.reverse
    }
    #[inline]
    pub fn is_always_start_anchored(&self) -> bool {}
    #[inline]
    pub fn look_matcher(&self) -> &LookMatcher {
        &self.0.look_matcher
    }
    #[inline]
    pub fn look_set_any(&self) -> LookSet {
        self.0.look_set_any
    }
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
impl StateBuilderMatches {
    pub(crate) fn into_nfa(mut self) -> StateBuilderNFA {}
    pub(crate) fn set_is_from_word(&mut self) {
        self.repr_vec().set_is_from_word()
    }
    pub(crate) fn set_is_half_crlf(&mut self) {
        self.repr_vec().set_is_half_crlf()
    }
    pub(crate) fn look_have(&self) -> LookSet {}
    pub(crate) fn set_look_have(&mut self, set: impl FnMut(LookSet) -> LookSet) {
        self.repr_vec().set_look_have(set)
    }
    pub(crate) fn add_match_pattern_id(&mut self, pid: PatternID) {}
    fn repr(&self) -> Repr<'_> {}
    fn repr_vec(&mut self) -> ReprVec<'_> {}
}
impl LookSet {
    #[inline]
    pub fn empty() -> LookSet {}
    #[inline]
    pub fn full() -> LookSet {}
    #[inline]
    pub fn singleton(look: Look) -> LookSet {}
    #[inline]
    pub fn len(self) -> usize {}
    #[inline]
    pub fn is_empty(self) -> bool {}
    #[inline]
    pub fn contains(self, look: Look) -> bool {}
    #[inline]
    pub fn contains_anchor(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_haystack(&self) -> bool {
        self.contains(Look::Start) || self.contains(Look::End)
    }
    #[inline]
    pub fn contains_anchor_line(&self) -> bool {
        self.contains(Look::StartLF) || self.contains(Look::EndLF)
            || self.contains(Look::StartCRLF) || self.contains(Look::EndCRLF)
    }
    #[inline]
    pub fn contains_anchor_lf(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_crlf(&self) -> bool {
        self.contains(Look::StartCRLF) || self.contains(Look::EndCRLF)
    }
    #[inline]
    pub fn contains_word(self) -> bool {
        self.contains_word_unicode() || self.contains_word_ascii()
    }
    #[inline]
    pub fn contains_word_unicode(self) -> bool {}
    #[inline]
    pub fn contains_word_ascii(self) -> bool {}
    #[inline]
    pub fn iter(self) -> LookSetIter {}
    #[inline]
    pub fn insert(self, look: Look) -> LookSet {}
    #[inline]
    pub fn set_insert(&mut self, look: Look) {}
    #[inline]
    pub fn remove(self, look: Look) -> LookSet {}
    #[inline]
    pub fn set_remove(&mut self, look: Look) {}
    #[inline]
    pub fn subtract(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_subtract(&mut self, other: LookSet) {}
    #[inline]
    pub fn union(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_union(&mut self, other: LookSet) {}
    #[inline]
    pub fn intersect(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_intersect(&mut self, other: LookSet) {}
    #[inline]
    pub fn read_repr(slice: &[u8]) -> LookSet {}
    #[inline]
    pub fn write_repr(self, slice: &mut [u8]) {}
    pub fn available(self) -> Result<(), UnicodeWordBoundaryError> {}
}
pub(crate) fn set_lookbehind_from_start(
    nfa: &thompson::NFA,
    start: &Start,
    builder: &mut StateBuilderMatches,
) {
    let rev = nfa.is_reverse();
    let lineterm = nfa.look_matcher().get_line_terminator();
    let lookset = nfa.look_set_any();
    match *start {
        Start::NonWordByte => {
            if lookset.contains_word() {
                builder
                    .set_look_have(|have| {
                        have.insert(Look::WordStartHalfAscii)
                            .insert(Look::WordStartHalfUnicode)
                    });
            }
        }
        Start::WordByte => {
            if lookset.contains_word() {
                builder.set_is_from_word();
            }
        }
        Start::Text => {
            if lookset.contains_anchor_haystack() {
                builder.set_look_have(|have| have.insert(Look::Start));
            }
            if lookset.contains_anchor_line() {
                builder
                    .set_look_have(|have| {
                        have.insert(Look::StartLF).insert(Look::StartCRLF)
                    });
            }
            if lookset.contains_word() {
                builder
                    .set_look_have(|have| {
                        have.insert(Look::WordStartHalfAscii)
                            .insert(Look::WordStartHalfUnicode)
                    });
            }
        }
        Start::LineLF => {
            if rev {
                if lookset.contains_anchor_crlf() {
                    builder.set_is_half_crlf();
                }
                if lookset.contains_anchor_line() {
                    builder.set_look_have(|have| have.insert(Look::StartLF));
                }
            } else {
                if lookset.contains_anchor_line() {
                    builder.set_look_have(|have| have.insert(Look::StartCRLF));
                }
            }
            if lookset.contains_anchor_line() && lineterm == b'\n' {
                builder.set_look_have(|have| have.insert(Look::StartLF));
            }
            if lookset.contains_word() {
                builder
                    .set_look_have(|have| {
                        have.insert(Look::WordStartHalfAscii)
                            .insert(Look::WordStartHalfUnicode)
                    });
            }
        }
        Start::LineCR => {
            if lookset.contains_anchor_crlf() {
                if rev {
                    builder.set_look_have(|have| have.insert(Look::StartCRLF));
                } else {
                    builder.set_is_half_crlf();
                }
            }
            if lookset.contains_anchor_line() && lineterm == b'\r' {
                builder.set_look_have(|have| have.insert(Look::StartLF));
            }
            if lookset.contains_word() {
                builder
                    .set_look_have(|have| {
                        have.insert(Look::WordStartHalfAscii)
                            .insert(Look::WordStartHalfUnicode)
                    });
            }
        }
        Start::CustomLineTerminator => {
            if lookset.contains_anchor_line() {
                builder.set_look_have(|have| have.insert(Look::StartLF));
            }
            if lookset.contains_word() {
                if utf8::is_word_byte(lineterm) {
                    builder.set_is_from_word();
                } else {
                    builder
                        .set_look_have(|have| {
                            have.insert(Look::WordStartHalfAscii)
                                .insert(Look::WordStartHalfUnicode)
                        });
                }
            }
        }
    }
}
#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn is_word_byte(b: u8) -> bool {
    const fn mkwordset() -> [bool; 256] {
        let mut set = [false; 256];
        set[b'_' as usize] = true;
        let mut byte = b'0';
        while byte <= b'9' {
            set[byte as usize] = true;
            byte += 1;
        }
        byte = b'A';
        while byte <= b'Z' {
            set[byte as usize] = true;
            byte += 1;
        }
        byte = b'a';
        while byte <= b'z' {
            set[byte as usize] = true;
            byte += 1;
        }
        set
    }
    const WORD: [bool; 256] = mkwordset();
    WORD[b as usize]
}
