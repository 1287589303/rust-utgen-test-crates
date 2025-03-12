use crate::buf::{IntoIter, UninitSlice};
use crate::{Buf, BufMut};
#[cfg(feature = "std")]
use std::io::IoSlice;
#[derive(Debug)]
pub struct Chain<T, U> {
    a: T,
    b: U,
}
#[derive(Debug)]
pub struct IntoIter<T> {
    inner: T,
}
impl<T, U> IntoIterator for Chain<T, U>
where
    T: Buf,
    U: Buf,
{
    type Item = u8;
    type IntoIter = IntoIter<Chain<T, U>>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}
