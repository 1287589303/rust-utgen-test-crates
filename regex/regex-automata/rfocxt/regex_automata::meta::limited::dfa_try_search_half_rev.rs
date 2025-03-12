use crate::{
    meta::error::{RetryError, RetryQuadraticError},
    HalfMatch, Input, MatchError,
};
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Debug)]
pub(crate) struct RetryQuadraticError(());
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HalfMatch {
    /// The pattern ID.
    pattern: PatternID,
    /// The offset of the match.
    ///
    /// For forward searches, the offset is exclusive. For reverse searches,
    /// the offset is inclusive.
    offset: usize,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Debug)]
pub(crate) enum RetryError {
    Quadratic(RetryQuadraticError),
    Fail(RetryFailError),
}
impl<'h> Input<'h> {
    #[inline]
    pub fn new<H: ?Sized + AsRef<[u8]>>(haystack: &'h H) -> Input<'h> {}
    #[inline]
    pub fn span<S: Into<Span>>(mut self, span: S) -> Input<'h> {}
    #[inline]
    pub fn range<R: RangeBounds<usize>>(mut self, range: R) -> Input<'h> {}
    #[inline]
    pub fn anchored(mut self, mode: Anchored) -> Input<'h> {}
    #[inline]
    pub fn earliest(mut self, yes: bool) -> Input<'h> {}
    #[inline]
    pub fn set_span<S: Into<Span>>(&mut self, span: S) {}
    #[inline]
    pub fn set_range<R: RangeBounds<usize>>(&mut self, range: R) {}
    #[inline]
    pub fn set_start(&mut self, start: usize) {}
    #[inline]
    pub fn set_end(&mut self, end: usize) {}
    #[inline]
    pub fn set_anchored(&mut self, mode: Anchored) {}
    #[inline]
    pub fn set_earliest(&mut self, yes: bool) {}
    #[inline]
    pub fn haystack(&self) -> &[u8] {
        self.haystack
    }
    #[inline]
    pub fn start(&self) -> usize {
        self.get_span().start
    }
    #[inline]
    pub fn end(&self) -> usize {
        self.get_span().end
    }
    #[inline]
    pub fn get_span(&self) -> Span {}
    #[inline]
    pub fn get_range(&self) -> Range<usize> {}
    #[inline]
    pub fn get_anchored(&self) -> Anchored {}
    #[inline]
    pub fn get_earliest(&self) -> bool {}
    #[inline]
    pub fn is_done(&self) -> bool {}
    #[inline]
    pub fn is_char_boundary(&self, offset: usize) -> bool {}
}
impl RetryQuadraticError {
    pub(crate) fn new() -> RetryQuadraticError {
        RetryQuadraticError(())
    }
}
impl HalfMatch {
    #[inline]
    pub fn new(pattern: PatternID, offset: usize) -> HalfMatch {
        HalfMatch { pattern, offset }
    }
    #[inline]
    pub fn must(pattern: usize, offset: usize) -> HalfMatch {}
    #[inline]
    pub fn pattern(&self) -> PatternID {}
    #[inline]
    pub fn offset(&self) -> usize {}
}
impl MatchError {
    pub fn new(kind: MatchErrorKind) -> MatchError {}
    pub fn kind(&self) -> &MatchErrorKind {}
    pub fn quit(byte: u8, offset: usize) -> MatchError {
        MatchError::new(MatchErrorKind::Quit {
            byte,
            offset,
        })
    }
    pub fn gave_up(offset: usize) -> MatchError {}
    pub fn haystack_too_long(len: usize) -> MatchError {}
    pub fn unsupported_anchored(mode: Anchored) -> MatchError {}
}
#[cfg(feature = "dfa-build")]
pub(crate) fn dfa_try_search_half_rev(
    dfa: &crate::dfa::dense::DFA<alloc::vec::Vec<u32>>,
    input: &Input<'_>,
    min_start: usize,
) -> Result<Option<HalfMatch>, RetryError> {
    use crate::dfa::Automaton;
    let mut mat = None;
    let mut sid = dfa.start_state_reverse(input)?;
    if input.start() == input.end() {
        dfa_eoi_rev(dfa, input, &mut sid, &mut mat)?;
        return Ok(mat);
    }
    let mut at = input.end() - 1;
    loop {
        sid = dfa.next_state(sid, input.haystack()[at]);
        if dfa.is_special_state(sid) {
            if dfa.is_match_state(sid) {
                let pattern = dfa.match_pattern(sid, 0);
                mat = Some(HalfMatch::new(pattern, at + 1));
            } else if dfa.is_dead_state(sid) {
                return Ok(mat);
            } else if dfa.is_quit_state(sid) {
                return Err(MatchError::quit(input.haystack()[at], at).into());
            }
        }
        if at == input.start() {
            break;
        }
        at -= 1;
        if at < min_start {
            trace!(
                "reached position {} which is before the previous literal \
				 match, quitting to avoid quadratic behavior",
                at,
            );
            return Err(RetryError::Quadratic(RetryQuadraticError::new()));
        }
    }
    let was_dead = dfa.is_dead_state(sid);
    dfa_eoi_rev(dfa, input, &mut sid, &mut mat)?;
    if at == input.start() && mat.map_or(false, |m| m.offset() > input.start())
        && !was_dead
    {
        trace!(
            "reached beginning of search at offset {} without hitting \
             a dead state, quitting to avoid potential false positive match",
            at,
        );
        return Err(RetryError::Quadratic(RetryQuadraticError::new()));
    }
    Ok(mat)
}
#[cfg(feature = "dfa-build")]
#[cfg_attr(feature = "perf-inline", inline(always))]
fn dfa_eoi_rev(
    dfa: &crate::dfa::dense::DFA<alloc::vec::Vec<u32>>,
    input: &Input<'_>,
    sid: &mut crate::util::primitives::StateID,
    mat: &mut Option<HalfMatch>,
) -> Result<(), MatchError> {
    use crate::dfa::Automaton;
    let sp = input.get_span();
    if sp.start > 0 {
        let byte = input.haystack()[sp.start - 1];
        *sid = dfa.next_state(*sid, byte);
        if dfa.is_match_state(*sid) {
            let pattern = dfa.match_pattern(*sid, 0);
            *mat = Some(HalfMatch::new(pattern, sp.start));
        } else if dfa.is_quit_state(*sid) {
            return Err(MatchError::quit(byte, sp.start - 1));
        }
    } else {
        *sid = dfa.next_eoi_state(*sid);
        if dfa.is_match_state(*sid) {
            let pattern = dfa.match_pattern(*sid, 0);
            *mat = Some(HalfMatch::new(pattern, 0));
        }
        debug_assert!(! dfa.is_quit_state(* sid));
    }
    Ok(())
}
