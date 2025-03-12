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
#[cfg(feature = "alloc")]
pub(crate) fn alloc_aligned_buffer<T>(size: usize) -> (Vec<u8>, usize) {
    let buf = vec![0; size];
    let align = core::mem::align_of::<T>();
    let address = buf.as_ptr().as_usize();
    if address % align == 0 {
        return (buf, 0);
    }
    let extra = align - 1;
    let mut buf = vec![0; size + extra];
    let address = buf.as_ptr().as_usize();
    if address % align == 0 {
        buf.truncate(size);
        return (buf, 0);
    }
    let padding = ((address & !(align - 1)).checked_add(align).unwrap())
        .checked_sub(address)
        .unwrap();
    assert!(padding <= 7, "padding of {} is bigger than 7", padding);
    assert!(
        padding <= extra, "padding of {} is bigger than extra {} bytes", padding, extra
    );
    buf.truncate(size + padding);
    assert_eq!(size + padding, buf.len());
    assert_eq!(
        0, buf[padding..].as_ptr().as_usize() % align,
        "expected end of initial padding to be aligned to {}", align,
    );
    (buf, padding)
}
