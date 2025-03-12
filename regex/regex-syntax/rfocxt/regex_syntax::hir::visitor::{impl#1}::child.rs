use alloc::{vec, vec::Vec};
use crate::hir::{self, Hir, HirKind};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Repetition {
    /// The minimum range of the repetition.
    ///
    /// Note that special cases like `?`, `+` and `*` all get translated into
    /// the ranges `{0,1}`, `{1,}` and `{0,}`, respectively.
    ///
    /// When `min` is zero, this expression can match the empty string
    /// regardless of what its sub-expression is.
    pub min: u32,
    /// The maximum range of the repetition.
    ///
    /// Note that when `max` is `None`, `min` acts as a lower bound but where
    /// there is no upper bound. For something like `x{5}` where the min and
    /// max are equivalent, `min` will be set to `5` and `max` will be set to
    /// `Some(5)`.
    pub max: Option<u32>,
    /// Whether this repetition operator is greedy or not. A greedy operator
    /// will match as much as it can. A non-greedy operator will match as
    /// little as it can.
    ///
    /// Typically, operators are greedy by default and are only non-greedy when
    /// a `?` suffix is used, e.g., `(expr)*` is greedy while `(expr)*?` is
    /// not. However, this can be inverted via the `U` "ungreedy" flag.
    pub greedy: bool,
    /// The expression being repeated.
    pub sub: Box<Hir>,
}
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
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Repetition {
    /// The span of this operation.
    pub span: Span,
    /// The actual operation.
    pub op: RepetitionOp,
    /// Whether this operation was applied greedily or not.
    pub greedy: bool,
    /// The regular expression under repetition.
    pub ast: Box<Ast>,
}
enum Frame<'a> {
    /// A stack frame allocated just before descending into a repetition
    /// operator's child node.
    Repetition(&'a hir::Repetition),
    /// A stack frame allocated just before descending into a capture's child
    /// node.
    Capture(&'a hir::Capture),
    /// The stack frame used while visiting every child node of a concatenation
    /// of expressions.
    Concat {
        /// The child node we are currently visiting.
        head: &'a Hir,
        /// The remaining child nodes to visit (which may be empty).
        tail: &'a [Hir],
    },
    /// The stack frame used while visiting every child node of an alternation
    /// of expressions.
    Alternation {
        /// The child node we are currently visiting.
        head: &'a Hir,
        /// The remaining child nodes to visit (which may be empty).
        tail: &'a [Hir],
    },
}
impl<'a> Frame<'a> {
    fn child(&self) -> &'a Hir {
        match *self {
            Frame::Repetition(rep) => &rep.sub,
            Frame::Capture(capture) => &capture.sub,
            Frame::Concat { head, .. } => head,
            Frame::Alternation { head, .. } => head,
        }
    }
}
