#[cfg(feature = "alloc")]
use crate::util::search::PatternSet;
use crate::{
    dfa::search,
    util::{
        empty, prefilter::Prefilter, primitives::{PatternID, StateID},
        search::{Anchored, HalfMatch, Input, MatchError},
        start,
    },
};
pub unsafe trait Automaton {
    fn next_state(&self, current: StateID, input: u8) -> StateID;
    unsafe fn next_state_unchecked(&self, current: StateID, input: u8) -> StateID;
    fn next_eoi_state(&self, current: StateID) -> StateID;
    fn start_state(&self, config: &start::Config) -> Result<StateID, StartError>;
    fn start_state_forward(&self, input: &Input<'_>) -> Result<StateID, MatchError> {
        let config = start::Config::from_input_forward(input);
        self.start_state(&config)
            .map_err(|err| match err {
                StartError::Quit { byte } => {
                    let offset = input
                        .start()
                        .checked_sub(1)
                        .expect("no quit in start without look-behind");
                    MatchError::quit(byte, offset)
                }
                StartError::UnsupportedAnchored { mode } => {
                    MatchError::unsupported_anchored(mode)
                }
            })
    }
    fn start_state_reverse(&self, input: &Input<'_>) -> Result<StateID, MatchError> {
        let config = start::Config::from_input_reverse(input);
        self.start_state(&config)
            .map_err(|err| match err {
                StartError::Quit { byte } => {
                    let offset = input.end();
                    MatchError::quit(byte, offset)
                }
                StartError::UnsupportedAnchored { mode } => {
                    MatchError::unsupported_anchored(mode)
                }
            })
    }
    #[inline]
    fn universal_start_state(&self, _mode: Anchored) -> Option<StateID> {
        None
    }
    fn is_special_state(&self, id: StateID) -> bool;
    fn is_dead_state(&self, id: StateID) -> bool;
    fn is_quit_state(&self, id: StateID) -> bool;
    fn is_match_state(&self, id: StateID) -> bool;
    fn is_start_state(&self, id: StateID) -> bool;
    fn is_accel_state(&self, id: StateID) -> bool;
    fn pattern_len(&self) -> usize;
    fn match_len(&self, id: StateID) -> usize;
    fn match_pattern(&self, id: StateID, index: usize) -> PatternID;
    fn has_empty(&self) -> bool;
    fn is_utf8(&self) -> bool;
    fn is_always_start_anchored(&self) -> bool;
    #[inline]
    fn accelerator(&self, _id: StateID) -> &[u8] {
        &[]
    }
    #[inline]
    fn get_prefilter(&self) -> Option<&Prefilter> {
        None
    }
    #[inline]
    fn try_search_fwd(
        &self,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, MatchError> {
        let utf8empty = self.has_empty() && self.is_utf8();
        let hm = match search::find_fwd(&self, input)? {
            None => return Ok(None),
            Some(hm) if !utf8empty => return Ok(Some(hm)),
            Some(hm) => hm,
        };
        empty::skip_splits_fwd(
            input,
            hm,
            hm.offset(),
            |input| {
                let got = search::find_fwd(&self, input)?;
                Ok(got.map(|hm| (hm, hm.offset())))
            },
        )
    }
    #[inline]
    fn try_search_rev(
        &self,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, MatchError> {
        let utf8empty = self.has_empty() && self.is_utf8();
        let hm = match search::find_rev(self, input)? {
            None => return Ok(None),
            Some(hm) if !utf8empty => return Ok(Some(hm)),
            Some(hm) => hm,
        };
        empty::skip_splits_rev(
            input,
            hm,
            hm.offset(),
            |input| {
                let got = search::find_rev(self, input)?;
                Ok(got.map(|hm| (hm, hm.offset())))
            },
        )
    }
    #[inline]
    fn try_search_overlapping_fwd(
        &self,
        input: &Input<'_>,
        state: &mut OverlappingState,
    ) -> Result<(), MatchError> {
        let utf8empty = self.has_empty() && self.is_utf8();
        search::find_overlapping_fwd(self, input, state)?;
        match state.get_match() {
            None => Ok(()),
            Some(_) if !utf8empty => Ok(()),
            Some(_) => {
                skip_empty_utf8_splits_overlapping(
                    input,
                    state,
                    |input, state| { search::find_overlapping_fwd(self, input, state) },
                )
            }
        }
    }
    #[inline]
    fn try_search_overlapping_rev(
        &self,
        input: &Input<'_>,
        state: &mut OverlappingState,
    ) -> Result<(), MatchError> {
        let utf8empty = self.has_empty() && self.is_utf8();
        search::find_overlapping_rev(self, input, state)?;
        match state.get_match() {
            None => Ok(()),
            Some(_) if !utf8empty => Ok(()),
            Some(_) => {
                skip_empty_utf8_splits_overlapping(
                    input,
                    state,
                    |input, state| { search::find_overlapping_rev(self, input, state) },
                )
            }
        }
    }
    #[cfg(feature = "alloc")]
    #[inline]
    fn try_which_overlapping_matches(
        &self,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) -> Result<(), MatchError> {
        let mut state = OverlappingState::start();
        while let Some(m) = {
            self.try_search_overlapping_fwd(input, &mut state)?;
            state.get_match()
        } {
            let _ = patset.insert(m.pattern());
            if patset.is_full() || input.get_earliest() {
                break;
            }
        }
        Ok(())
    }
}
pub(crate) trait U16 {
    fn as_usize(self) -> usize;
    fn low_u8(self) -> u8;
    fn high_u8(self) -> u8;
}
pub(crate) trait U32 {
    fn as_usize(self) -> usize;
    fn low_u8(self) -> u8;
    fn low_u16(self) -> u16;
    fn high_u16(self) -> u16;
}
pub(crate) trait Usize {
    fn as_u8(self) -> u8;
    fn as_u16(self) -> u16;
    fn as_u32(self) -> u32;
    fn as_u64(self) -> u64;
}
pub(crate) trait U8 {
    fn as_usize(self) -> usize;
}
pub(crate) trait U64 {
    fn as_usize(self) -> usize;
    fn low_u8(self) -> u8;
    fn low_u16(self) -> u16;
    fn low_u32(self) -> u32;
    fn high_u32(self) -> u32;
}
pub(crate) trait I32 {
    fn as_usize(self) -> usize;
    fn to_bits(self) -> u32;
    fn from_bits(n: u32) -> i32;
}
pub(crate) trait Pointer {
    fn as_usize(self) -> usize;
}
#[cfg(feature = "alloc")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatternSet {
    /// The number of patterns set to 'true' in this set.
    len: usize,
    /// A map from PatternID to boolean of whether a pattern matches or not.
    ///
    /// This should probably be a bitset, but it's probably unlikely to matter
    /// much in practice.
    ///
    /// The main downside of this representation (and similarly for a bitset)
    /// is that iteration scales with the capacity of the set instead of
    /// the length of the set. This doesn't seem likely to be a problem in
    /// practice.
    ///
    /// Another alternative is to just use a 'SparseSet' for this. It does use
    /// more memory (quite a bit more), but that seems fine I think compared
    /// to the memory being used by the regex engine. The real hiccup with
    /// it is that it yields pattern IDs in the order they were inserted.
    /// Which is actually kind of nice, but at the time of writing, pattern
    /// IDs are yielded in ascending order in the regex crate RegexSet API.
    /// If we did change to 'SparseSet', we could provide an additional
    /// 'iter_match_order' iterator, but keep the ascending order one for
    /// compatibility.
    which: alloc::boxed::Box<[bool]>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
unsafe impl<'a, A: Automaton + ?Sized> Automaton for &'a A {
    #[inline]
    fn next_state(&self, current: StateID, input: u8) -> StateID {}
    #[inline]
    unsafe fn next_state_unchecked(&self, current: StateID, input: u8) -> StateID {}
    #[inline]
    fn next_eoi_state(&self, current: StateID) -> StateID {}
    #[inline]
    fn start_state(&self, config: &start::Config) -> Result<StateID, StartError> {}
    #[inline]
    fn start_state_forward(&self, input: &Input<'_>) -> Result<StateID, MatchError> {}
    #[inline]
    fn start_state_reverse(&self, input: &Input<'_>) -> Result<StateID, MatchError> {}
    #[inline]
    fn universal_start_state(&self, mode: Anchored) -> Option<StateID> {}
    #[inline]
    fn is_special_state(&self, id: StateID) -> bool {}
    #[inline]
    fn is_dead_state(&self, id: StateID) -> bool {}
    #[inline]
    fn is_quit_state(&self, id: StateID) -> bool {}
    #[inline]
    fn is_match_state(&self, id: StateID) -> bool {}
    #[inline]
    fn is_start_state(&self, id: StateID) -> bool {}
    #[inline]
    fn is_accel_state(&self, id: StateID) -> bool {}
    #[inline]
    fn pattern_len(&self) -> usize {}
    #[inline]
    fn match_len(&self, id: StateID) -> usize {}
    #[inline]
    fn match_pattern(&self, id: StateID, index: usize) -> PatternID {}
    #[inline]
    fn has_empty(&self) -> bool {}
    #[inline]
    fn is_utf8(&self) -> bool {}
    #[inline]
    fn is_always_start_anchored(&self) -> bool {}
    #[inline]
    fn accelerator(&self, id: StateID) -> &[u8] {}
    #[inline]
    fn get_prefilter(&self) -> Option<&Prefilter> {}
    #[inline]
    fn try_search_fwd(
        &self,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, MatchError> {}
    #[inline]
    fn try_search_rev(
        &self,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, MatchError> {}
    #[inline]
    fn try_search_overlapping_fwd(
        &self,
        input: &Input<'_>,
        state: &mut OverlappingState,
    ) -> Result<(), MatchError> {}
    #[inline]
    fn try_search_overlapping_rev(
        &self,
        input: &Input<'_>,
        state: &mut OverlappingState,
    ) -> Result<(), MatchError> {}
    #[cfg(feature = "alloc")]
    #[inline]
    fn try_which_overlapping_matches(
        &self,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) -> Result<(), MatchError> {
        (**self).try_which_overlapping_matches(input, patset)
    }
}
