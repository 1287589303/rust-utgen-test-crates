use alloc::{vec, vec::Vec};
use crate::hir::{self, Hir, HirKind};
struct HeapVisitor<'a> {
    /// A stack of `Hir` nodes. This is roughly analogous to the call stack
    /// used in a typical recursive visitor.
    stack: Vec<(&'a Hir, Frame<'a>)>,
}
#[derive(Clone, Eq, PartialEq)]
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    props: Properties,
}
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capture {
    /// The capture index of the capture.
    pub index: u32,
    /// The name of the capture, if it exists.
    pub name: Option<Box<str>>,
    /// The expression inside the capturing group, which may be empty.
    pub sub: Box<Hir>,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HirKind {
    /// The empty regular expression, which matches everything, including the
    /// empty string.
    Empty,
    /// A literalstring that matches exactly these bytes.
    Literal(Literal),
    /// A single character class that matches any of the characters in the
    /// class. A class can either consist of Unicode scalar values as
    /// characters, or it can use bytes.
    ///
    /// A class may be empty. In which case, it matches nothing.
    Class(Class),
    /// A look-around assertion. A look-around match always has zero length.
    Look(Look),
    /// A repetition operation applied to a sub-expression.
    Repetition(Repetition),
    /// A capturing group, which contains a sub-expression.
    Capture(Capture),
    /// A concatenation of expressions.
    ///
    /// A concatenation matches only if each of its sub-expressions match one
    /// after the other.
    ///
    /// Concatenations are guaranteed by `Hir`'s smart constructors to always
    /// have at least two sub-expressions.
    Concat(Vec<Hir>),
    /// An alternation of expressions.
    ///
    /// An alternation matches only if at least one of its sub-expressions
    /// match. If multiple sub-expressions match, then the leftmost is
    /// preferred.
    ///
    /// Alternations are guaranteed by `Hir`'s smart constructors to always
    /// have at least two sub-expressions.
    Alternation(Vec<Hir>),
}
enum Frame<'a> {
    /// A stack frame allocated just before descending into a repetition
    /// operator's child node.
    Repetition(&'a ast::Repetition),
    /// A stack frame allocated just before descending into a group's child
    /// node.
    Group(&'a ast::Group),
    /// The stack frame used while visiting every child node of a concatenation
    /// of expressions.
    Concat {
        /// The child node we are currently visiting.
        head: &'a Ast,
        /// The remaining child nodes to visit (which may be empty).
        tail: &'a [Ast],
    },
    /// The stack frame used while visiting every child node of an alternation
    /// of expressions.
    Alternation {
        /// The child node we are currently visiting.
        head: &'a Ast,
        /// The remaining child nodes to visit (which may be empty).
        tail: &'a [Ast],
    },
}
impl<'a> HeapVisitor<'a> {
    fn new() -> HeapVisitor<'a> {}
    fn visit<V: Visitor>(
        &mut self,
        mut hir: &'a Hir,
        mut visitor: V,
    ) -> Result<V::Output, V::Err> {}
    fn induct(&mut self, hir: &'a Hir) -> Option<Frame<'a>> {
        match *hir.kind() {
            HirKind::Repetition(ref x) => Some(Frame::Repetition(x)),
            HirKind::Capture(ref x) => Some(Frame::Capture(x)),
            HirKind::Concat(ref x) if x.is_empty() => None,
            HirKind::Concat(ref x) => {
                Some(Frame::Concat {
                    head: &x[0],
                    tail: &x[1..],
                })
            }
            HirKind::Alternation(ref x) if x.is_empty() => None,
            HirKind::Alternation(ref x) => {
                Some(Frame::Alternation {
                    head: &x[0],
                    tail: &x[1..],
                })
            }
            _ => None,
        }
    }
    fn pop(&self, induct: Frame<'a>) -> Option<Frame<'a>> {}
}
impl Hir {
    pub fn kind(&self) -> &HirKind {
        &self.kind
    }
    pub fn into_kind(mut self) -> HirKind {}
    pub fn properties(&self) -> &Properties {}
    fn into_parts(mut self) -> (HirKind, Properties) {}
}
