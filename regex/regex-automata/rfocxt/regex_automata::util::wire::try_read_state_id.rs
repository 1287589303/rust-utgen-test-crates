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
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl DeserializeError {
    pub(crate) fn generic(msg: &'static str) -> DeserializeError {}
    pub(crate) fn buffer_too_small(what: &'static str) -> DeserializeError {
        DeserializeError(DeserializeErrorKind::BufferTooSmall {
            what,
        })
    }
    fn invalid_usize(what: &'static str) -> DeserializeError {}
    fn version_mismatch(expected: u32, found: u32) -> DeserializeError {}
    fn endian_mismatch(expected: u32, found: u32) -> DeserializeError {}
    fn alignment_mismatch(alignment: usize, address: usize) -> DeserializeError {}
    fn label_mismatch(expected: &'static str) -> DeserializeError {}
    fn arithmetic_overflow(what: &'static str) -> DeserializeError {}
    fn pattern_id_error(err: PatternIDError, what: &'static str) -> DeserializeError {}
    pub(crate) fn state_id_error(
        err: StateIDError,
        what: &'static str,
    ) -> DeserializeError {}
}
pub(crate) fn try_read_state_id(
    slice: &[u8],
    what: &'static str,
) -> Result<(StateID, usize), DeserializeError> {
    if slice.len() < StateID::SIZE {
        return Err(DeserializeError::buffer_too_small(what));
    }
    read_state_id(slice, what)
}
pub(crate) fn read_state_id(
    slice: &[u8],
    what: &'static str,
) -> Result<(StateID, usize), DeserializeError> {
    let bytes: [u8; StateID::SIZE] = slice[..StateID::SIZE].try_into().unwrap();
    let sid = StateID::from_ne_bytes(bytes)
        .map_err(|err| DeserializeError::state_id_error(err, what))?;
    Ok((sid, StateID::SIZE))
}
