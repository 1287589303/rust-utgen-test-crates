use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
pub(crate) trait I32 {
    fn as_usize(self) -> usize;
    fn to_bits(self) -> u32;
    fn from_bits(n: u32) -> i32;
}
fn write_vari32(data: &mut Vec<u8>, n: i32) {
    let mut un = n.to_bits() << 1;
    if n < 0 {
        un = !un;
    }
    write_varu32(data, un)
}
fn write_varu32(data: &mut Vec<u8>, mut n: u32) {
    while n >= 0b1000_0000 {
        data.push(n.low_u8() | 0b1000_0000);
        n >>= 7;
    }
    data.push(n.low_u8());
}
