use alloc::{
    format, string::{String, ToString},
    vec, vec::Vec,
};
use crate::{ast, hir};
fn repeat_char(c: char, count: usize) -> String {
    core::iter::repeat(c).take(count).collect()
}
