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
pub(crate) struct Accels<A> {
    /// A length prefixed slice of contiguous accelerators. See the top comment
    /// in this module for more details on how we can jump from a DFA's state
    /// ID to an accelerator in this list.
    ///
    /// The first 4 bytes always correspond to the number of accelerators
    /// that follow.
    accels: A,
}
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
#[cfg(feature = "dfa-build")]
impl Accels<Vec<AccelTy>> {
    pub fn empty() -> Accels<Vec<AccelTy>> {}
    pub fn add(&mut self, accel: Accel) {
        self.accels.extend_from_slice(&accel.as_accel_tys());
        let len = self.len();
        self.set_len(len + 1);
    }
    fn set_len(&mut self, new_len: usize) {}
}
impl Accel {
    #[cfg(feature = "dfa-build")]
    pub fn new() -> Accel {}
    pub fn from_slice(mut slice: &[u8]) -> Result<Accel, DeserializeError> {}
    fn from_bytes(bytes: [u8; 4]) -> Result<Accel, DeserializeError> {}
    fn from_bytes_unchecked(bytes: [u8; 4]) -> Accel {}
    #[cfg(feature = "dfa-build")]
    pub fn add(&mut self, byte: u8) -> bool {}
    pub fn len(&self) -> usize {}
    #[cfg(feature = "dfa-build")]
    pub fn is_empty(&self) -> bool {}
    fn needles(&self) -> &[u8] {}
    #[cfg(feature = "dfa-build")]
    fn contains(&self, byte: u8) -> bool {}
    #[cfg(feature = "dfa-build")]
    fn as_accel_tys(&self) -> [AccelTy; 2] {
        assert_eq!(ACCEL_CAP, 8);
        let first = AccelTy::from_ne_bytes(self.bytes[0..4].try_into().unwrap());
        let second = AccelTy::from_ne_bytes(self.bytes[4..8].try_into().unwrap());
        [first, second]
    }
}
