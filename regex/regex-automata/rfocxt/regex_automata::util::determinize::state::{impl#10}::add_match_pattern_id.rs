use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
struct ReprVec<'a>(&'a mut Vec<u8>);
struct Repr<'a>(&'a [u8]);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
impl<'a> ReprVec<'a> {
    fn set_is_match(&mut self) {
        self.0[0] |= 1 << 0;
    }
    fn set_has_pattern_ids(&mut self) {
        self.0[0] |= 1 << 1;
    }
    fn set_is_from_word(&mut self) {}
    fn set_is_half_crlf(&mut self) {}
    fn look_have(&self) -> LookSet {}
    fn look_need(&self) -> LookSet {}
    fn set_look_have(&mut self, mut set: impl FnMut(LookSet) -> LookSet) {}
    fn set_look_need(&mut self, mut set: impl FnMut(LookSet) -> LookSet) {}
    fn add_match_pattern_id(&mut self, pid: PatternID) {
        if !self.repr().has_pattern_ids() {
            if pid == PatternID::ZERO {
                self.set_is_match();
                return;
            }
            self.0.extend(core::iter::repeat(0).take(PatternID::SIZE));
            self.set_has_pattern_ids();
            if self.repr().is_match() {
                write_u32(self.0, 0)
            } else {
                self.set_is_match();
            }
        }
        write_u32(self.0, pid.as_u32());
    }
    fn close_match_pattern_ids(&mut self) {}
    fn add_nfa_state_id(&mut self, prev: &mut StateID, sid: StateID) {}
    fn repr(&self) -> Repr<'_> {
        Repr(self.0.as_slice())
    }
}
impl<'a> Repr<'a> {
    fn is_match(&self) -> bool {
        self.0[0] & (1 << 0) > 0
    }
    fn has_pattern_ids(&self) -> bool {
        self.0[0] & (1 << 1) > 0
    }
    fn is_from_word(&self) -> bool {}
    fn is_half_crlf(&self) -> bool {}
    fn look_have(&self) -> LookSet {}
    fn look_need(&self) -> LookSet {}
    fn match_len(&self) -> usize {}
    fn match_pattern(&self, index: usize) -> PatternID {}
    fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {}
    fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, mut f: F) {}
    fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, mut f: F) {}
    fn pattern_offset_end(&self) -> usize {}
    fn encoded_pattern_len(&self) -> usize {}
}
fn write_u32(dst: &mut Vec<u8>, n: u32) {
    use crate::util::wire::NE;
    let start = dst.len();
    dst.extend(core::iter::repeat(0).take(mem::size_of::<u32>()));
    NE::write_u32(n, &mut dst[start..]);
}
