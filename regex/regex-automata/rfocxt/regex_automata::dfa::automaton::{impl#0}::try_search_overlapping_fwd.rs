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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverlappingState {
    /// The match reported by the most recent overlapping search to use this
    /// state.
    ///
    /// If a search does not find any matches, then it is expected to clear
    /// this value.
    pub(crate) mat: Option<HalfMatch>,
    /// The state ID of the state at which the search was in when the call
    /// terminated. When this is a match state, `last_match` must be set to a
    /// non-None value.
    ///
    /// A `None` value indicates the start state of the corresponding
    /// automaton. We cannot use the actual ID, since any one automaton may
    /// have many start states, and which one is in use depends on several
    /// search-time factors.
    pub(crate) id: Option<StateID>,
    /// The position of the search.
    ///
    /// When `id` is None (i.e., we are starting a search), this is set to
    /// the beginning of the search as given by the caller regardless of its
    /// current value. Subsequent calls to an overlapping search pick up at
    /// this offset.
    pub(crate) at: usize,
    /// The index into the matching patterns of the next match to report if the
    /// current state is a match state. Note that this may be 1 greater than
    /// the total number of matches to report for the current match state. (In
    /// which case, no more matches should be reported at the current position
    /// and the search should advance to the next position.)
    pub(crate) next_match_index: Option<usize>,
    /// This is set to true when a reverse overlapping search has entered its
    /// EOI transitions.
    ///
    /// This isn't used in a forward search because it knows to stop once the
    /// position exceeds the end of the search range. In a reverse search,
    /// since we use unsigned offsets, we don't "know" once we've gone past
    /// `0`. So the only way to detect it is with this extra flag. The reverse
    /// overlapping search knows to terminate specifically after it has
    /// reported all matches after following the EOI transition.
    pub(crate) rev_eoi: bool,
}
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
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
    ) -> Result<(), MatchError> {
        (**self).try_search_overlapping_fwd(input, state)
    }
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
    ) -> Result<(), MatchError> {}
}
