use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
struct ReprVec<'a>(&'a mut Vec<u8>);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl<'a> ReprVec<'a> {
    fn set_is_match(&mut self) {}
    fn set_has_pattern_ids(&mut self) {}
    fn set_is_from_word(&mut self) {}
    fn set_is_half_crlf(&mut self) {}
    fn look_have(&self) -> LookSet {}
    fn look_need(&self) -> LookSet {}
    fn set_look_have(&mut self, mut set: impl FnMut(LookSet) -> LookSet) {}
    fn set_look_need(&mut self, mut set: impl FnMut(LookSet) -> LookSet) {}
    fn add_match_pattern_id(&mut self, pid: PatternID) {}
    fn close_match_pattern_ids(&mut self) {}
    fn add_nfa_state_id(&mut self, prev: &mut StateID, sid: StateID) {
        let delta = sid.as_i32() - prev.as_i32();
        write_vari32(self.0, delta);
        *prev = sid;
    }
    fn repr(&self) -> Repr<'_> {}
}
fn write_vari32(data: &mut Vec<u8>, n: i32) {
    let mut un = n.to_bits() << 1;
    if n < 0 {
        un = !un;
    }
    write_varu32(data, un)
}
