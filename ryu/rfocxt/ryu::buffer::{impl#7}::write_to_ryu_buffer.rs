use crate::raw;
use core::mem::MaybeUninit;
use core::{slice, str};
#[cfg(feature = "no-panic")]
use no_panic::no_panic;
const NAN: &str = "NaN";
const INFINITY: &str = "inf";
const NEG_INFINITY: &str = "-inf";
pub trait Sealed: Copy {
    fn is_nonfinite(self) -> bool;
    fn format_nonfinite(self) -> &'static str;
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize;
}
impl Sealed for f64 {
    #[inline]
    fn is_nonfinite(self) -> bool {}
    #[cold]
    #[cfg_attr(feature = "no-panic", inline)]
    fn format_nonfinite(self) -> &'static str {}
    #[inline]
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
        raw::format64(self, result)
    }
}
#[must_use]
#[cfg_attr(feature = "no-panic", no_panic)]
pub unsafe fn format64(f: f64, result: *mut u8) -> usize {
    let bits = f.to_bits();
    let sign = ((bits >> (DOUBLE_MANTISSA_BITS + DOUBLE_EXPONENT_BITS)) & 1) != 0;
    let ieee_mantissa = bits & ((1u64 << DOUBLE_MANTISSA_BITS) - 1);
    let ieee_exponent = (bits >> DOUBLE_MANTISSA_BITS) as u32
        & ((1u32 << DOUBLE_EXPONENT_BITS) - 1);
    let mut index = 0isize;
    if sign {
        *result = b'-';
        index += 1;
    }
    if ieee_exponent == 0 && ieee_mantissa == 0 {
        ptr::copy_nonoverlapping(b"0.0".as_ptr(), result.offset(index), 3);
        return sign as usize + 3;
    }
    let v = d2d(ieee_mantissa, ieee_exponent);
    let length = d2s::decimal_length17(v.mantissa) as isize;
    let k = v.exponent as isize;
    let kk = length + k;
    debug_assert!(k >= - 324);
    if 0 <= k && kk <= 16 {
        write_mantissa_long(v.mantissa, result.offset(index + length));
        for i in length..kk {
            *result.offset(index + i) = b'0';
        }
        *result.offset(index + kk) = b'.';
        *result.offset(index + kk + 1) = b'0';
        index as usize + kk as usize + 2
    } else if 0 < kk && kk <= 16 {
        write_mantissa_long(v.mantissa, result.offset(index + length + 1));
        ptr::copy(result.offset(index + 1), result.offset(index), kk as usize);
        *result.offset(index + kk) = b'.';
        index as usize + length as usize + 1
    } else if -5 < kk && kk <= 0 {
        *result.offset(index) = b'0';
        *result.offset(index + 1) = b'.';
        let offset = 2 - kk;
        for i in 2..offset {
            *result.offset(index + i) = b'0';
        }
        write_mantissa_long(v.mantissa, result.offset(index + length + offset));
        index as usize + length as usize + offset as usize
    } else if length == 1 {
        *result.offset(index) = b'0' + v.mantissa as u8;
        *result.offset(index + 1) = b'e';
        index as usize + 2 + write_exponent3(kk - 1, result.offset(index + 2))
    } else {
        write_mantissa_long(v.mantissa, result.offset(index + length + 1));
        *result.offset(index) = *result.offset(index + 1);
        *result.offset(index + 1) = b'.';
        *result.offset(index + length + 1) = b'e';
        index as usize + length as usize + 2
            + write_exponent3(kk - 1, result.offset(index + length + 2))
    }
}
