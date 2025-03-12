type Result<T> = core::result::Result<T, Error>;
use core::cell::{Cell, RefCell};
use alloc::{boxed::Box, string::ToString, vec, vec::Vec};
use crate::{
    ast::{self, Ast, Span, Visitor},
    either::Either, hir::{self, Error, ErrorKind, Hir, HirKind},
    unicode::{self, ClassQuery},
};
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum ClassAsciiKind {
    /// `[0-9A-Za-z]`
    Alnum,
    /// `[A-Za-z]`
    Alpha,
    /// `[\x00-\x7F]`
    Ascii,
    /// `[ \t]`
    Blank,
    /// `[\x00-\x1F\x7F]`
    Cntrl,
    /// `[0-9]`
    Digit,
    /// `[!-~]`
    Graph,
    /// `[a-z]`
    Lower,
    /// `[ -~]`
    Print,
    /// `[!-/:-@\[-`{-~]`
    Punct,
    /// `[\t\n\v\f\r ]`
    Space,
    /// `[A-Z]`
    Upper,
    /// `[0-9A-Za-z_]`
    Word,
    /// `[0-9A-Fa-f]`
    Xdigit,
}
fn ascii_class_as_chars(
    kind: &ast::ClassAsciiKind,
) -> impl Iterator<Item = (char, char)> {
    ascii_class(kind).map(|(s, e)| (char::from(s), char::from(e)))
}
fn ascii_class(kind: &ast::ClassAsciiKind) -> impl Iterator<Item = (u8, u8)> {
    use crate::ast::ClassAsciiKind::*;
    let slice: &'static [(u8, u8)] = match *kind {
        Alnum => &[(b'0', b'9'), (b'A', b'Z'), (b'a', b'z')],
        Alpha => &[(b'A', b'Z'), (b'a', b'z')],
        Ascii => &[(b'\x00', b'\x7F')],
        Blank => &[(b'\t', b'\t'), (b' ', b' ')],
        Cntrl => &[(b'\x00', b'\x1F'), (b'\x7F', b'\x7F')],
        Digit => &[(b'0', b'9')],
        Graph => &[(b'!', b'~')],
        Lower => &[(b'a', b'z')],
        Print => &[(b' ', b'~')],
        Punct => &[(b'!', b'/'), (b':', b'@'), (b'[', b'`'), (b'{', b'~')],
        Space => {
            &[
                (b'\t', b'\t'),
                (b'\n', b'\n'),
                (b'\x0B', b'\x0B'),
                (b'\x0C', b'\x0C'),
                (b'\r', b'\r'),
                (b' ', b' '),
            ]
        }
        Upper => &[(b'A', b'Z')],
        Word => &[(b'0', b'9'), (b'A', b'Z'), (b'_', b'_'), (b'a', b'z')],
        Xdigit => &[(b'0', b'9'), (b'A', b'F'), (b'a', b'f')],
    };
    slice.iter().copied()
}
