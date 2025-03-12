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
#[derive(Debug)]
pub struct DeserializeError(DeserializeErrorKind);
impl<A: AsRef<[AccelTy]>> Accels<A> {
    #[cfg(feature = "alloc")]
    pub fn to_owned(&self) -> Accels<alloc::vec::Vec<AccelTy>> {}
    pub fn as_ref(&self) -> Accels<&[AccelTy]> {}
    pub fn as_bytes(&self) -> &[u8] {
        let accels = self.accels.as_ref();
        unsafe {
            core::slice::from_raw_parts(
                accels.as_ptr().cast::<u8>(),
                accels.len() * ACCEL_TY_SIZE,
            )
        }
    }
    pub fn memory_usage(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub fn needles(&self, i: usize) -> &[u8] {}
    pub fn len(&self) -> usize {}
    fn get(&self, i: usize) -> Option<Accel> {}
    fn iter(&self) -> IterAccels<'_, A> {}
    pub fn write_to<E: Endian>(&self, dst: &mut [u8]) -> Result<usize, SerializeError> {}
    pub fn validate(&self) -> Result<(), DeserializeError> {
        for chunk in self.as_bytes()[ACCEL_TY_SIZE..].chunks(ACCEL_CAP) {
            let _ = Accel::from_slice(chunk)?;
        }
        Ok(())
    }
    pub fn write_to_len(&self) -> usize {}
}
impl Accel {
    #[cfg(feature = "dfa-build")]
    pub fn new() -> Accel {}
    pub fn from_slice(mut slice: &[u8]) -> Result<Accel, DeserializeError> {
        slice = &slice[..core::cmp::min(ACCEL_LEN, slice.len())];
        let bytes = slice
            .try_into()
            .map_err(|_| DeserializeError::buffer_too_small("accelerator"))?;
        Accel::from_bytes(bytes)
    }
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
    fn as_accel_tys(&self) -> [AccelTy; 2] {}
}
