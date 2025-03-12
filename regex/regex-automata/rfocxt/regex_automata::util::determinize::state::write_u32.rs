use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
pub(crate) trait Endian {
    fn write_u16(n: u16, dst: &mut [u8]);
    fn write_u32(n: u32, dst: &mut [u8]);
    fn write_u128(n: u128, dst: &mut [u8]);
}
fn write_u32(dst: &mut Vec<u8>, n: u32) {
    use crate::util::wire::NE;
    let start = dst.len();
    dst.extend(core::iter::repeat(0).take(mem::size_of::<u32>()));
    NE::write_u32(n, &mut dst[start..]);
}
