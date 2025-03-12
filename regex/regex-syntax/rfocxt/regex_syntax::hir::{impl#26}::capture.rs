use core::{char, cmp};
use alloc::{
    boxed::Box, format, string::{String, ToString},
    vec, vec::Vec,
};
use crate::{
    ast::Span, hir::interval::{Interval, IntervalSet, IntervalSetIter},
    unicode,
};
pub use crate::{
    hir::visitor::{visit, Visitor},
    unicode::CaseFoldError,
};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Properties(Box<PropertiesI>);
#[derive(Clone, Eq, PartialEq)]
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    props: Properties,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capture {
    /// The capture index of the capture.
    pub index: u32,
    /// The name of the capture, if it exists.
    pub name: Option<Box<str>>,
    /// The expression inside the capturing group, which may be empty.
    pub sub: Box<Hir>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct PropertiesI {
    minimum_len: Option<usize>,
    maximum_len: Option<usize>,
    look_set: LookSet,
    look_set_prefix: LookSet,
    look_set_suffix: LookSet,
    look_set_prefix_any: LookSet,
    look_set_suffix_any: LookSet,
    utf8: bool,
    explicit_captures_len: usize,
    static_explicit_captures_len: Option<usize>,
    literal: bool,
    alternation_literal: bool,
}
impl Properties {
    fn empty() -> Properties {}
    fn literal(lit: &Literal) -> Properties {}
    fn class(class: &Class) -> Properties {}
    fn look(look: Look) -> Properties {}
    fn repetition(rep: &Repetition) -> Properties {}
    fn capture(capture: &Capture) -> Properties {
        let p = capture.sub.properties();
        Properties(
            Box::new(PropertiesI {
                explicit_captures_len: p.explicit_captures_len().saturating_add(1),
                static_explicit_captures_len: p
                    .static_explicit_captures_len()
                    .map(|len| len.saturating_add(1)),
                literal: false,
                alternation_literal: false,
                ..*p.0.clone()
            }),
        )
    }
    fn concat(concat: &[Hir]) -> Properties {}
    fn alternation(alts: &[Hir]) -> Properties {}
}
impl Hir {
    pub fn kind(&self) -> &HirKind {}
    pub fn into_kind(mut self) -> HirKind {}
    pub fn properties(&self) -> &Properties {
        &self.props
    }
    fn into_parts(mut self) -> (HirKind, Properties) {}
}
