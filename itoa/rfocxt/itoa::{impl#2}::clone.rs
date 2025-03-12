use core::hint;
use core::mem::MaybeUninit;
use core::{ptr, slice, str};
#[cfg(feature = "no-panic")]
use no_panic::no_panic;
const DEC_DIGITS_LUT: [u8; 200] = *b"\
      0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";
pub struct Buffer {
    bytes: [MaybeUninit<u8>; i128::MAX_STR_LEN],
}
impl Clone for Buffer {
    #[inline]
    #[allow(clippy::non_canonical_clone_impl)]
    fn clone(&self) -> Self {
        Buffer::new()
    }
}
impl Buffer {
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn new() -> Buffer {
        let bytes = [MaybeUninit::<u8>::uninit(); i128::MAX_STR_LEN];
        Buffer { bytes }
    }
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format<I: Integer>(&mut self, i: I) -> &str {}
}
