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
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
pub(crate) fn read_pattern_id(
    slice: &[u8],
    what: &'static str,
) -> Result<(PatternID, usize), DeserializeError> {
    let bytes: [u8; PatternID::SIZE] = slice[..PatternID::SIZE].try_into().unwrap();
    let pid = PatternID::from_ne_bytes(bytes)
        .map_err(|err| DeserializeError::pattern_id_error(err, what))?;
    Ok((pid, PatternID::SIZE))
}
