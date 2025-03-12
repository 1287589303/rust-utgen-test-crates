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
#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn u32s_to_state_ids(slice: &[u32]) -> &[StateID] {
    unsafe { core::slice::from_raw_parts(slice.as_ptr().cast::<StateID>(), slice.len()) }
}
