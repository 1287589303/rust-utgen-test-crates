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
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Debug)]
pub struct DeserializeError(DeserializeErrorKind);
pub(crate) fn read_state_id(
    slice: &[u8],
    what: &'static str,
) -> Result<(StateID, usize), DeserializeError> {
    let bytes: [u8; StateID::SIZE] = slice[..StateID::SIZE].try_into().unwrap();
    let sid = StateID::from_ne_bytes(bytes)
        .map_err(|err| DeserializeError::state_id_error(err, what))?;
    Ok((sid, StateID::SIZE))
}
