pub use crate::buf::{Buf, BufMut};
pub use crate::bytes::Bytes;
pub use crate::bytes_mut::BytesMut;
#[derive(Debug, PartialEq, Eq)]
pub struct TryGetError {
    /// The number of bytes necessary to get the value
    pub requested: usize,
    /// The number of bytes available in the buffer
    pub available: usize,
}
#[cfg(feature = "std")]
impl From<TryGetError> for std::io::Error {
    fn from(error: TryGetError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, error)
    }
}
