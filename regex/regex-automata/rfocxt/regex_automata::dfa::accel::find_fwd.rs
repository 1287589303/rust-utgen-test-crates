type AccelTy = u32;
#[cfg(feature = "dfa-build")]
use alloc::{vec, vec::Vec};
use crate::util::{
    int::Pointer, memchr, wire::{self, DeserializeError, Endian, SerializeError},
};
const ACCEL_TY_SIZE: usize = core::mem::size_of::<AccelTy>();
const ACCEL_LEN: usize = 4;
const ACCEL_CAP: usize = 8;
#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn find_fwd(needles: &[u8], haystack: &[u8], at: usize) -> Option<usize> {
    let bs = needles;
    let i = match needles.len() {
        1 => memchr::memchr(bs[0], &haystack[at..])?,
        2 => memchr::memchr2(bs[0], bs[1], &haystack[at..])?,
        3 => memchr::memchr3(bs[0], bs[1], bs[2], &haystack[at..])?,
        0 => panic!("cannot find with empty needles"),
        n => panic!("invalid needles length: {}", n),
    };
    Some(at + i)
}
#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn memchr3(n1: u8, n2: u8, n3: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == n1 || b == n2 || b == n3)
}
#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn memchr2(n1: u8, n2: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == n1 || b == n2)
}
#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn memchr(n1: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == n1)
}
