use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
pub(crate) trait I32 {
    fn as_usize(self) -> usize;
    fn to_bits(self) -> u32;
    fn from_bits(n: u32) -> i32;
}
struct Repr<'a>(&'a [u8]);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl<'a> Repr<'a> {
    fn is_match(&self) -> bool {}
    fn has_pattern_ids(&self) -> bool {}
    fn is_from_word(&self) -> bool {}
    fn is_half_crlf(&self) -> bool {}
    fn look_have(&self) -> LookSet {}
    fn look_need(&self) -> LookSet {}
    fn match_len(&self) -> usize {}
    fn match_pattern(&self, index: usize) -> PatternID {}
    fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {}
    fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, mut f: F) {}
    fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, mut f: F) {
        let mut sids = &self.0[self.pattern_offset_end()..];
        let mut prev = 0i32;
        while !sids.is_empty() {
            let (delta, nr) = read_vari32(sids);
            sids = &sids[nr..];
            let sid = prev + delta;
            prev = sid;
            f(StateID::new_unchecked(sid.as_usize()))
        }
    }
    fn pattern_offset_end(&self) -> usize {
        let encoded = self.encoded_pattern_len();
        if encoded == 0 {
            return 9;
        }
        encoded.checked_mul(4).unwrap().checked_add(13).unwrap()
    }
    fn encoded_pattern_len(&self) -> usize {}
}
fn read_vari32(data: &[u8]) -> (i32, usize) {
    let (un, i) = read_varu32(data);
    let mut n = i32::from_bits(un >> 1);
    if un & 1 != 0 {
        n = !n;
    }
    (n, i)
}
