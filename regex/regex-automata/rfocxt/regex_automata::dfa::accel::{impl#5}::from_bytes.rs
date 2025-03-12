type AccelTy = u32;
#[cfg(feature = "dfa-build")]
use alloc::{vec, vec::Vec};
use crate::util::{
    int::Pointer, memchr, wire::{self, DeserializeError, Endian, SerializeError},
};
const ACCEL_TY_SIZE: usize = core::mem::size_of::<AccelTy>();
const ACCEL_LEN: usize = 4;
const ACCEL_CAP: usize = 8;
#[derive(Clone)]
pub(crate) struct Accel {
    /// The first byte is the length. Subsequent bytes are the accelerated
    /// bytes.
    ///
    /// Note that we make every accelerator 8 bytes as a slightly wasteful
    /// way of making sure alignment is always correct for state ID sizes of
    /// 1, 2, 4 and 8. This should be okay since accelerated states aren't
    /// particularly common, especially when Unicode is enabled.
    bytes: [u8; ACCEL_CAP],
}
#[derive(Debug)]
pub struct DeserializeError(DeserializeErrorKind);
impl Accel {
    #[cfg(feature = "dfa-build")]
    pub fn new() -> Accel {}
    pub fn from_slice(mut slice: &[u8]) -> Result<Accel, DeserializeError> {}
    fn from_bytes(bytes: [u8; 4]) -> Result<Accel, DeserializeError> {
        if usize::from(bytes[0]) >= ACCEL_LEN {
            return Err(
                DeserializeError::generic(
                    "accelerator bytes cannot have length more than 3",
                ),
            );
        }
        Ok(Accel::from_bytes_unchecked(bytes))
    }
    fn from_bytes_unchecked(bytes: [u8; 4]) -> Accel {
        Accel {
            bytes: [bytes[0], bytes[1], bytes[2], bytes[3], 0, 0, 0, 0],
        }
    }
    #[cfg(feature = "dfa-build")]
    pub fn add(&mut self, byte: u8) -> bool {}
    pub fn len(&self) -> usize {}
    #[cfg(feature = "dfa-build")]
    pub fn is_empty(&self) -> bool {}
    fn needles(&self) -> &[u8] {}
    #[cfg(feature = "dfa-build")]
    fn contains(&self, byte: u8) -> bool {}
    #[cfg(feature = "dfa-build")]
    fn as_accel_tys(&self) -> [AccelTy; 2] {}
}
impl DeserializeError {
    pub(crate) fn generic(msg: &'static str) -> DeserializeError {
        DeserializeError(DeserializeErrorKind::Generic {
            msg,
        })
    }
    pub(crate) fn buffer_too_small(what: &'static str) -> DeserializeError {}
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
