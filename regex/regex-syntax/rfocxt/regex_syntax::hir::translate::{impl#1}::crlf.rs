type Result<T> = core::result::Result<T, Error>;
use core::cell::{Cell, RefCell};
use alloc::{boxed::Box, string::ToString, vec, vec::Vec};
use crate::{
    ast::{self, Ast, Span, Visitor},
    either::Either, hir::{self, Error, ErrorKind, Hir, HirKind},
    unicode::{self, ClassQuery},
};
#[derive(Clone, Debug)]
pub struct TranslatorBuilder {
    utf8: bool,
    line_terminator: u8,
    flags: Flags,
}
#[derive(Clone, Copy, Debug, Default)]
struct Flags {
    case_insensitive: Option<bool>,
    multi_line: Option<bool>,
    dot_matches_new_line: Option<bool>,
    swap_greed: Option<bool>,
    unicode: Option<bool>,
    crlf: Option<bool>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Flags {
    /// The span of this group of flags.
    pub span: Span,
    /// A sequence of flag items. Each item is either a flag or a negation
    /// operator.
    pub items: Vec<FlagsItem>,
}
impl TranslatorBuilder {
    pub fn new() -> TranslatorBuilder {}
    pub fn build(&self) -> Translator {}
    pub fn utf8(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn line_terminator(&mut self, byte: u8) -> &mut TranslatorBuilder {}
    pub fn case_insensitive(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn multi_line(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn crlf(&mut self, yes: bool) -> &mut TranslatorBuilder {
        self.flags.crlf = if yes { Some(true) } else { None };
        self
    }
    pub fn swap_greed(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn unicode(&mut self, yes: bool) -> &mut TranslatorBuilder {}
}
