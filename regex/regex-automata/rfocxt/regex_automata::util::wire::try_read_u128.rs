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
#[derive(Debug)]
pub struct DeserializeError(DeserializeErrorKind);
pub(crate) fn try_read_u128(
    slice: &[u8],
    what: &'static str,
) -> Result<(u128, usize), DeserializeError> {
    check_slice_len(slice, size_of::<u128>(), what)?;
    Ok((read_u128(slice), size_of::<u128>()))
}
pub(crate) fn read_u128(slice: &[u8]) -> u128 {
    let bytes: [u8; 16] = slice[..size_of::<u128>()].try_into().unwrap();
    u128::from_ne_bytes(bytes)
}
pub(crate) fn check_slice_len<T>(
    slice: &[T],
    at_least_len: usize,
    what: &'static str,
) -> Result<(), DeserializeError> {
    if slice.len() < at_least_len {
        return Err(DeserializeError::buffer_too_small(what));
    }
    Ok(())
}
