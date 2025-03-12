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
pub(crate) fn try_read_u16_as_usize(
    slice: &[u8],
    what: &'static str,
) -> Result<(usize, usize), DeserializeError> {
    try_read_u16(slice, what)
        .and_then(|(n, nr)| {
            usize::try_from(n)
                .map(|n| (n, nr))
                .map_err(|_| DeserializeError::invalid_usize(what))
        })
}
pub(crate) fn try_read_u16(
    slice: &[u8],
    what: &'static str,
) -> Result<(u16, usize), DeserializeError> {
    check_slice_len(slice, size_of::<u16>(), what)?;
    Ok((read_u16(slice), size_of::<u16>()))
}
