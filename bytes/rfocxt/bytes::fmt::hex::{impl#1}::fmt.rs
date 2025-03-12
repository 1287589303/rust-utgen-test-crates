use core::fmt::{Formatter, LowerHex, Result, UpperHex};
use super::BytesRef;
use crate::{Bytes, BytesMut};
struct BytesRef<'a>(&'a [u8]);
impl UpperHex for BytesRef<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for &b in self.0 {
            write!(f, "{:02X}", b)?;
        }
        Ok(())
    }
}
