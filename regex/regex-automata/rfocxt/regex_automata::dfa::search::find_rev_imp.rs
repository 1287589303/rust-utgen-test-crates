use crate::{
    dfa::{accel, automaton::{Automaton, OverlappingState}},
    util::{
        prefilter::Prefilter, primitives::StateID,
        search::{Anchored, HalfMatch, Input, Span},
    },
    MatchError,
};
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
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
pub struct StateID(SmallIndex);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
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
#[cfg_attr(feature = "perf-inline", inline(always))]
fn find_rev_imp<A: Automaton + ?Sized>(
    dfa: &A,
    input: &Input<'_>,
    earliest: bool,
) -> Result<Option<HalfMatch>, MatchError> {
    let mut mat = None;
    let mut sid = init_rev(dfa, input)?;
    if input.start() == input.end() {
        eoi_rev(dfa, input, &mut sid, &mut mat)?;
        return Ok(mat);
    }
    let mut at = input.end() - 1;
    macro_rules! next_unchecked {
        ($sid:expr, $at:expr) => {
            { let byte = * input.haystack().get_unchecked($at); dfa
            .next_state_unchecked($sid, byte) }
        };
    }
    loop {
        let mut prev_sid;
        while at >= input.start() {
            prev_sid = unsafe { next_unchecked!(sid, at) };
            if dfa.is_special_state(prev_sid) || at <= input.start().saturating_add(3) {
                core::mem::swap(&mut prev_sid, &mut sid);
                break;
            }
            at -= 1;
            sid = unsafe { next_unchecked!(prev_sid, at) };
            if dfa.is_special_state(sid) {
                break;
            }
            at -= 1;
            prev_sid = unsafe { next_unchecked!(sid, at) };
            if dfa.is_special_state(prev_sid) {
                core::mem::swap(&mut prev_sid, &mut sid);
                break;
            }
            at -= 1;
            sid = unsafe { next_unchecked!(prev_sid, at) };
            if dfa.is_special_state(sid) {
                break;
            }
            at -= 1;
        }
        if dfa.is_special_state(sid) {
            if dfa.is_start_state(sid) {
                if dfa.is_accel_state(sid) {
                    let needles = dfa.accelerator(sid);
                    at = accel::find_rev(needles, input.haystack(), at)
                        .map(|i| i + 1)
                        .unwrap_or(input.start());
                }
            } else if dfa.is_match_state(sid) {
                let pattern = dfa.match_pattern(sid, 0);
                mat = Some(HalfMatch::new(pattern, at + 1));
                if earliest {
                    return Ok(mat);
                }
                if dfa.is_accel_state(sid) {
                    let needles = dfa.accelerator(sid);
                    at = accel::find_rev(needles, input.haystack(), at)
                        .map(|i| i + 1)
                        .unwrap_or(input.start());
                }
            } else if dfa.is_accel_state(sid) {
                let needles = dfa.accelerator(sid);
                at = accel::find_rev(needles, input.haystack(), at)
                    .map(|i| i + 1)
                    .unwrap_or(input.start());
            } else if dfa.is_dead_state(sid) {
                return Ok(mat);
            } else {
                return Err(MatchError::quit(input.haystack()[at], at));
            }
        }
        if at == input.start() {
            break;
        }
        at -= 1;
    }
    eoi_rev(dfa, input, &mut sid, &mut mat)?;
    Ok(mat)
}
#[cfg_attr(feature = "perf-inline", inline(always))]
fn eoi_rev<A: Automaton + ?Sized>(
    dfa: &A,
    input: &Input<'_>,
    sid: &mut StateID,
    mat: &mut Option<HalfMatch>,
) -> Result<(), MatchError> {
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
    }
    Ok(())
}
#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn find_rev(needles: &[u8], haystack: &[u8], at: usize) -> Option<usize> {
    let bs = needles;
    match needles.len() {
        1 => memchr::memrchr(bs[0], &haystack[..at]),
        2 => memchr::memrchr2(bs[0], bs[1], &haystack[..at]),
        3 => memchr::memrchr3(bs[0], bs[1], bs[2], &haystack[..at]),
        0 => panic!("cannot find with empty needles"),
        n => panic!("invalid needles length: {}", n),
    }
}
#[cfg_attr(feature = "perf-inline", inline(always))]
fn init_rev<A: Automaton + ?Sized>(
    dfa: &A,
    input: &Input<'_>,
) -> Result<StateID, MatchError> {
    let sid = dfa.start_state_reverse(input)?;
    debug_assert!(! dfa.is_match_state(sid));
    Ok(sid)
}
