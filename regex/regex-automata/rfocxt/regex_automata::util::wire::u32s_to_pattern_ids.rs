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
pub struct PatternID(SmallIndex);
#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn u32s_to_pattern_ids(slice: &[u32]) -> &[PatternID] {
    unsafe {
        core::slice::from_raw_parts(slice.as_ptr().cast::<PatternID>(), slice.len())
    }
}
