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
pub(crate) fn read_u128(slice: &[u8]) -> u128 {
    let bytes: [u8; 16] = slice[..size_of::<u128>()].try_into().unwrap();
    u128::from_ne_bytes(bytes)
}
