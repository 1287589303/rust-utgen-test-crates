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
impl DeserializeError {
    pub(crate) fn generic(msg: &'static str) -> DeserializeError {}
    pub(crate) fn buffer_too_small(what: &'static str) -> DeserializeError {}
    fn invalid_usize(what: &'static str) -> DeserializeError {}
    fn version_mismatch(expected: u32, found: u32) -> DeserializeError {}
    fn endian_mismatch(expected: u32, found: u32) -> DeserializeError {}
    fn alignment_mismatch(alignment: usize, address: usize) -> DeserializeError {
        DeserializeError(DeserializeErrorKind::AlignmentMismatch {
            alignment,
            address,
        })
    }
    fn label_mismatch(expected: &'static str) -> DeserializeError {}
    fn arithmetic_overflow(what: &'static str) -> DeserializeError {}
    fn pattern_id_error(err: PatternIDError, what: &'static str) -> DeserializeError {}
    pub(crate) fn state_id_error(
        err: StateIDError,
        what: &'static str,
    ) -> DeserializeError {}
}
pub(crate) fn check_alignment<T>(slice: &[u8]) -> Result<(), DeserializeError> {
    let alignment = core::mem::align_of::<T>();
    let address = slice.as_ptr().as_usize();
    if address % alignment == 0 {
        return Ok(());
    }
    Err(DeserializeError::alignment_mismatch(alignment, address))
}
