type AccelTy = u32;
#[cfg(feature = "dfa-build")]
use alloc::{vec, vec::Vec};
use crate::util::{
    int::Pointer, memchr, wire::{self, DeserializeError, Endian, SerializeError},
};
const ACCEL_TY_SIZE: usize = core::mem::size_of::<AccelTy>();
const ACCEL_LEN: usize = 4;
const ACCEL_CAP: usize = 8;
#[derive(Debug)]
struct IterAccels<'a, A: AsRef<[AccelTy]>> {
    accels: &'a Accels<A>,
    i: usize,
}
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
impl<'a, A: AsRef<[AccelTy]>> Iterator for IterAccels<'a, A> {
    type Item = Accel;
    fn next(&mut self) -> Option<Accel> {
        let accel = self.accels.get(self.i)?;
        self.i += 1;
        Some(accel)
    }
}
impl<A: AsRef<[AccelTy]>> Accels<A> {
    #[cfg(feature = "alloc")]
    pub fn to_owned(&self) -> Accels<alloc::vec::Vec<AccelTy>> {}
    pub fn as_ref(&self) -> Accels<&[AccelTy]> {}
    pub fn as_bytes(&self) -> &[u8] {}
    pub fn memory_usage(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub fn needles(&self, i: usize) -> &[u8] {}
    pub fn len(&self) -> usize {}
    fn get(&self, i: usize) -> Option<Accel> {
        if i >= self.len() {
            return None;
        }
        let offset = ACCEL_TY_SIZE + i * ACCEL_CAP;
        let accel = Accel::from_slice(&self.as_bytes()[offset..])
            .expect("Accels must contain valid accelerators");
        Some(accel)
    }
    fn iter(&self) -> IterAccels<'_, A> {}
    pub fn write_to<E: Endian>(&self, dst: &mut [u8]) -> Result<usize, SerializeError> {}
    pub fn validate(&self) -> Result<(), DeserializeError> {}
    pub fn write_to_len(&self) -> usize {}
}
