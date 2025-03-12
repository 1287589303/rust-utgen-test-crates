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
impl<A: AsRef<[AccelTy]>> Accels<A> {
    #[cfg(feature = "alloc")]
    pub fn to_owned(&self) -> Accels<alloc::vec::Vec<AccelTy>> {}
    pub fn as_ref(&self) -> Accels<&[AccelTy]> {
        Accels {
            accels: self.accels.as_ref(),
        }
    }
    pub fn as_bytes(&self) -> &[u8] {}
    pub fn memory_usage(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub fn needles(&self, i: usize) -> &[u8] {}
    pub fn len(&self) -> usize {}
    fn get(&self, i: usize) -> Option<Accel> {}
    fn iter(&self) -> IterAccels<'_, A> {}
    pub fn write_to<E: Endian>(&self, dst: &mut [u8]) -> Result<usize, SerializeError> {}
    pub fn validate(&self) -> Result<(), DeserializeError> {}
    pub fn write_to_len(&self) -> usize {}
}
