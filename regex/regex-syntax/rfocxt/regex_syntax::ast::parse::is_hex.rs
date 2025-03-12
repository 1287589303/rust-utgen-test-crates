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
fn is_hex(c: char) -> bool {
    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
}
