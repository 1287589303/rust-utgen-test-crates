use crate::lib::*;
use crate::ser::{Error, Serialize, SerializeTuple, Serializer};
#[cfg(any(feature = "std", not(no_core_net)))]
const DEC_DIGITS_LUT: &[u8] = b"\
      0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";
#[cfg_attr(
    not(no_diagnostic_namespace),
    diagnostic::on_unimplemented(
        note = "for local types consider adding `#[derive(serde::Serialize)]` to your `{Self}` type",
        note = "for types from other crates check whether the crate offers a `serde` feature flag",
    )
)]
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
pub(super) struct Buf<'a> {
    bytes: &'a mut [u8],
    offset: usize,
}
#[cfg(any(feature = "std", not(no_core_net)))]
impl Serialize for net::Ipv4Addr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            const MAX_LEN: usize = 15;
            debug_assert_eq!(MAX_LEN, "101.102.103.104".len());
            let mut buf = [b'.'; MAX_LEN];
            let mut written = format_u8(self.octets()[0], &mut buf);
            for oct in &self.octets()[1..] {
                written += format_u8(*oct, &mut buf[written + 1..]) + 1;
            }
            let buf = unsafe { str::from_utf8_unchecked(&buf[..written]) };
            serializer.serialize_str(buf)
        } else {
            self.octets().serialize(serializer)
        }
    }
}
impl<'a> Buf<'a> {
    pub fn new(bytes: &'a mut [u8]) -> Self {
        Buf { bytes, offset: 0 }
    }
    pub fn as_str(&self) -> &str {
        let slice = &self.bytes[..self.offset];
        unsafe { str::from_utf8_unchecked(slice) }
    }
}
