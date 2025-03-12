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
#[derive(Debug)]
enum DeserializeErrorKind {
    Generic { msg: &'static str },
    BufferTooSmall { what: &'static str },
    InvalidUsize { what: &'static str },
    VersionMismatch { expected: u32, found: u32 },
    EndianMismatch { expected: u32, found: u32 },
    AlignmentMismatch { alignment: usize, address: usize },
    LabelMismatch { expected: &'static str },
    ArithmeticOverflow { what: &'static str },
    PatternID { err: PatternIDError, what: &'static str },
    StateID { err: StateIDError, what: &'static str },
}
impl DeserializeError {
    pub(crate) fn generic(msg: &'static str) -> DeserializeError {}
    pub(crate) fn buffer_too_small(what: &'static str) -> DeserializeError {}
    fn invalid_usize(what: &'static str) -> DeserializeError {}
    fn version_mismatch(expected: u32, found: u32) -> DeserializeError {}
    fn endian_mismatch(expected: u32, found: u32) -> DeserializeError {
        DeserializeError(DeserializeErrorKind::EndianMismatch {
            expected,
            found,
        })
    }
    fn alignment_mismatch(alignment: usize, address: usize) -> DeserializeError {}
    fn label_mismatch(expected: &'static str) -> DeserializeError {}
    fn arithmetic_overflow(what: &'static str) -> DeserializeError {}
    fn pattern_id_error(err: PatternIDError, what: &'static str) -> DeserializeError {}
    pub(crate) fn state_id_error(
        err: StateIDError,
        what: &'static str,
    ) -> DeserializeError {}
}
