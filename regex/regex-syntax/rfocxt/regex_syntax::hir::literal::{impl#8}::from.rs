use core::{cmp, mem, num::NonZeroUsize};
use alloc::{vec, vec::Vec};
use crate::hir::{self, Hir};
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Literal {
    bytes: Vec<u8>,
    exact: bool,
}
impl From<char> for Literal {
    fn from(ch: char) -> Literal {
        use alloc::string::ToString;
        Literal::exact(ch.encode_utf8(&mut [0; 4]).to_string())
    }
}
