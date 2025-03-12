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
impl Accel {
    #[cfg(feature = "dfa-build")]
    pub fn new() -> Accel {}
    pub fn from_slice(mut slice: &[u8]) -> Result<Accel, DeserializeError> {}
    fn from_bytes(bytes: [u8; 4]) -> Result<Accel, DeserializeError> {}
    fn from_bytes_unchecked(bytes: [u8; 4]) -> Accel {}
    #[cfg(feature = "dfa-build")]
    pub fn add(&mut self, byte: u8) -> bool {}
    pub fn len(&self) -> usize {
        usize::from(self.bytes[0])
    }
    #[cfg(feature = "dfa-build")]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn needles(&self) -> &[u8] {}
    #[cfg(feature = "dfa-build")]
    fn contains(&self, byte: u8) -> bool {}
    #[cfg(feature = "dfa-build")]
    fn as_accel_tys(&self) -> [AccelTy; 2] {}
}
