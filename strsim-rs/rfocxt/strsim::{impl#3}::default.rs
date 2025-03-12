pub type HammingResult = Result<usize, StrSimError>;
use std::char;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;
use std::mem;
use std::str::Chars;
#[derive(Clone, Copy, PartialEq, Eq)]
struct RowId {
    val: isize,
}
impl Default for RowId {
    fn default() -> Self {
        Self { val: -1 }
    }
}
