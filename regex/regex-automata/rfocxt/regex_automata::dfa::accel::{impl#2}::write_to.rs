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
#[derive(Debug)]
pub struct SerializeError {
    /// The name of the thing that a buffer is too small for.
    ///
    /// Currently, the only kind of serialization error is one that is
    /// committed by a caller: providing a destination buffer that is too
    /// small to fit the serialized object. This makes sense conceptually,
    /// since every valid inhabitant of a type should be serializable.
    ///
    /// This is somewhat exposed in the public API of this crate. For example,
    /// the `to_bytes_{big,little}_endian` APIs return a `Vec<u8>` and are
    /// guaranteed to never panic or error. This is only possible because the
    /// implementation guarantees that it will allocate a `Vec<u8>` that is
    /// big enough.
    ///
    /// In summary, if a new serialization error kind needs to be added, then
    /// it will need careful consideration.
    what: &'static str,
}
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
    pub fn len(&self) -> usize {
        usize::try_from(self.accels.as_ref()[0]).unwrap()
    }
    fn get(&self, i: usize) -> Option<Accel> {}
    fn iter(&self) -> IterAccels<'_, A> {}
    pub fn write_to<E: Endian>(&self, dst: &mut [u8]) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        assert_eq!(
            nwrite % ACCEL_TY_SIZE, 0,
            "expected accelerator bytes written to be a multiple of {}", ACCEL_TY_SIZE,
        );
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("accelerators"));
        }
        E::write_u32(AccelTy::try_from(self.len()).unwrap(), dst);
        dst[ACCEL_TY_SIZE..nwrite]
            .copy_from_slice(&self.as_bytes()[ACCEL_TY_SIZE..nwrite]);
        Ok(nwrite)
    }
    pub fn validate(&self) -> Result<(), DeserializeError> {}
    pub fn write_to_len(&self) -> usize {
        self.as_bytes().len()
    }
}
impl SerializeError {
    pub(crate) fn buffer_too_small(what: &'static str) -> SerializeError {
        SerializeError { what }
    }
}
