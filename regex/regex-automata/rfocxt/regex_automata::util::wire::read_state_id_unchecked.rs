#[cfg(target_endian = "little")]
pub(crate) type NE = LE;
#[cfg(target_endian = "big")]
pub(crate) type NE = BE;
use core::{cmp, mem::size_of};
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use crate::util::{
    int::Pointer, primitives::{PatternID, PatternIDError, StateID, StateIDError},
};
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
pub(crate) fn read_state_id_unchecked(slice: &[u8]) -> (StateID, usize) {
    let sid = StateID::from_ne_bytes_unchecked(
        slice[..StateID::SIZE].try_into().unwrap(),
    );
    (sid, StateID::SIZE)
}
