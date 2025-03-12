type Result<T> = core::result::Result<T, ast::Error>;
use core::{
    borrow::Borrow, cell::{Cell, RefCell},
    mem,
};
use alloc::{
    boxed::Box, string::{String, ToString},
    vec, vec::Vec,
};
use crate::{
    ast::{self, Ast, Position, Span},
    either::Either, is_escapeable_character, is_meta_character,
};
fn is_capture_char(c: char, first: bool) -> bool {
    if first {
        c == '_' || c.is_alphabetic()
    } else {
        c == '_' || c == '.' || c == '[' || c == ']' || c.is_alphanumeric()
    }
}
