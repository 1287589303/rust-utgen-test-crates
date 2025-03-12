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
pub(crate) trait Endian {
    fn write_u16(n: u16, dst: &mut [u8]);
    fn write_u32(n: u32, dst: &mut [u8]);
    fn write_u128(n: u128, dst: &mut [u8]);
}
pub(crate) enum LE {}
impl Endian for LE {
    fn write_u16(n: u16, dst: &mut [u8]) {}
    fn write_u32(n: u32, dst: &mut [u8]) {}
    fn write_u128(n: u128, dst: &mut [u8]) {
        dst[..16].copy_from_slice(&n.to_le_bytes());
    }
}
