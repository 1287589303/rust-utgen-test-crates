use crate::buf::{IntoIter, UninitSlice};
use crate::{Buf, BufMut};
#[cfg(feature = "std")]
use std::io::IoSlice;
#[derive(Debug)]
pub struct Chain<T, U> {
    a: T,
    b: U,
}
impl<T, U> Chain<T, U> {
    pub(crate) fn new(a: T, b: U) -> Chain<T, U> {}
    pub fn first_ref(&self) -> &T {}
    pub fn first_mut(&mut self) -> &mut T {}
    pub fn last_ref(&self) -> &U {
        &self.b
    }
    pub fn last_mut(&mut self) -> &mut U {}
    pub fn into_inner(self) -> (T, U) {}
}
