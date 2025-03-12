use crate::lib::*;
use crate::ser::{Error, Serialize, SerializeTuple, Serializer};
#[cfg(any(feature = "std", not(no_core_net)))]
const DEC_DIGITS_LUT: &[u8] = b"\
      0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";
#[cfg(any(feature = "std", not(no_core_net)))]
#[inline]
fn format_u8(mut n: u8, out: &mut [u8]) -> usize {
    if n >= 100 {
        let d1 = ((n % 100) << 1) as usize;
        n /= 100;
        out[0] = b'0' + n;
        out[1] = DEC_DIGITS_LUT[d1];
        out[2] = DEC_DIGITS_LUT[d1 + 1];
        3
    } else if n >= 10 {
        let d1 = (n << 1) as usize;
        out[0] = DEC_DIGITS_LUT[d1];
        out[1] = DEC_DIGITS_LUT[d1 + 1];
        2
    } else {
        out[0] = b'0' + n;
        1
    }
}
