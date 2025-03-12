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
impl SerializeError {
    pub(crate) fn buffer_too_small(what: &'static str) -> SerializeError {
        SerializeError { what }
    }
}
pub(crate) fn write_label(label: &str, dst: &mut [u8]) -> Result<usize, SerializeError> {
    let nwrite = write_label_len(label);
    if dst.len() < nwrite {
        return Err(SerializeError::buffer_too_small("label"));
    }
    dst[..label.len()].copy_from_slice(label.as_bytes());
    for i in 0..(nwrite - label.len()) {
        dst[label.len() + i] = 0;
    }
    assert_eq!(nwrite % 4, 0);
    Ok(nwrite)
}
pub(crate) fn write_label_len(label: &str) -> usize {
    if label.len() > 255 {
        panic!("label must not be longer than 255 bytes");
    }
    if label.as_bytes().iter().position(|&b| b == 0).is_some() {
        panic!("label must not contain NUL bytes");
    }
    let label_len = label.len() + 1;
    label_len + padding_len(label_len)
}
