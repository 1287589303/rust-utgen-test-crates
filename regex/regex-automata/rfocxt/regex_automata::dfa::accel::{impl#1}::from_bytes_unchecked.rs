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
pub struct DeserializeError(DeserializeErrorKind);
impl<'a> Accels<&'a [AccelTy]> {
    pub fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(Accels<&'a [AccelTy]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr().as_usize();
        let (accel_len, _) = wire::try_read_u32_as_usize(slice, "accelerators length")?;
        let accel_tys_len = wire::add(
            wire::mul(accel_len, 2, "total number of accelerator accel_tys")?,
            1,
            "total number of accel_tys",
        )?;
        let accel_tys_bytes_len = wire::mul(
            ACCEL_TY_SIZE,
            accel_tys_len,
            "total number of bytes in accelerators",
        )?;
        wire::check_slice_len(slice, accel_tys_bytes_len, "accelerators")?;
        wire::check_alignment::<AccelTy>(slice)?;
        let accel_tys = &slice[..accel_tys_bytes_len];
        slice = &slice[accel_tys_bytes_len..];
        let accels = unsafe {
            core::slice::from_raw_parts(
                accel_tys.as_ptr().cast::<AccelTy>(),
                accel_tys_len,
            )
        };
        Ok((Accels { accels }, slice.as_ptr().as_usize() - slice_start))
    }
}
pub(crate) fn check_alignment<T>(slice: &[u8]) -> Result<(), DeserializeError> {
    let alignment = core::mem::align_of::<T>();
    let address = slice.as_ptr().as_usize();
    if address % alignment == 0 {
        return Ok(());
    }
    Err(DeserializeError::alignment_mismatch(alignment, address))
}
pub(crate) fn mul(
    a: usize,
    b: usize,
    what: &'static str,
) -> Result<usize, DeserializeError> {
    match a.checked_mul(b) {
        Some(c) => Ok(c),
        None => Err(DeserializeError::arithmetic_overflow(what)),
    }
}
pub(crate) fn try_read_u32_as_usize(
    slice: &[u8],
    what: &'static str,
) -> Result<(usize, usize), DeserializeError> {
    try_read_u32(slice, what)
        .and_then(|(n, nr)| {
            usize::try_from(n)
                .map(|n| (n, nr))
                .map_err(|_| DeserializeError::invalid_usize(what))
        })
}
pub(crate) fn check_slice_len<T>(
    slice: &[T],
    at_least_len: usize,
    what: &'static str,
) -> Result<(), DeserializeError> {
    if slice.len() < at_least_len {
        return Err(DeserializeError::buffer_too_small(what));
    }
    Ok(())
}
pub(crate) fn add(
    a: usize,
    b: usize,
    what: &'static str,
) -> Result<usize, DeserializeError> {
    match a.checked_add(b) {
        Some(c) => Ok(c),
        None => Err(DeserializeError::arithmetic_overflow(what)),
    }
}
