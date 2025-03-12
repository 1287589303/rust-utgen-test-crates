use core::{cmp, mem, num::NonZeroUsize};
use alloc::{vec, vec::Vec};
use crate::hir::{self, Hir};
#[derive(Clone, Eq, PartialEq)]
pub struct Seq {
    /// The members of this seq.
    ///
    /// When `None`, the seq represents all possible literals. That is, it
    /// prevents one from making assumptions about specific literals in the
    /// seq, and forces one to treat it as if any literal might be in the seq.
    ///
    /// Note that `Some(vec![])` is valid and corresponds to the empty seq of
    /// literals, i.e., a regex that can never match. For example, `[a&&b]`.
    /// It is distinct from `Some(vec![""])`, which corresponds to the seq
    /// containing an empty string, which matches at every position.
    literals: Option<Vec<Literal>>,
}
#[derive(Debug)]
struct PreferenceTrie {
    /// The states in this trie. The index of a state in this vector is its ID.
    states: Vec<State>,
    /// This vec indicates which states are match states. It always has
    /// the same length as `states` and is indexed by the same state ID.
    /// A state with identifier `sid` is a match state if and only if
    /// `matches[sid].is_some()`. The option contains the index of the literal
    /// corresponding to the match. The index is offset by 1 so that it fits in
    /// a NonZeroUsize.
    matches: Vec<Option<NonZeroUsize>>,
    /// The index to allocate to the next literal added to this trie. Starts at
    /// 1 and increments by 1 for every literal successfully added to the trie.
    next_literal_index: usize,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Literal {
    /// The span of this literal.
    pub span: Span,
    /// The kind of this literal.
    pub kind: LiteralKind,
    /// The Unicode scalar value corresponding to this literal.
    pub c: char,
}
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Literal {
    bytes: Vec<u8>,
    exact: bool,
}
#[derive(Clone, Eq, PartialEq)]
pub struct Literal(pub Box<[u8]>);
impl Seq {
    #[inline]
    pub fn empty() -> Seq {}
    #[inline]
    pub fn infinite() -> Seq {}
    #[inline]
    pub fn singleton(lit: Literal) -> Seq {}
    #[inline]
    pub fn new<I, B>(it: I) -> Seq
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {}
    #[inline]
    pub fn literals(&self) -> Option<&[Literal]> {}
    #[inline]
    pub fn push(&mut self, lit: Literal) {}
    #[inline]
    pub fn make_inexact(&mut self) {}
    #[inline]
    pub fn make_infinite(&mut self) {}
    #[inline]
    pub fn cross_forward(&mut self, other: &mut Seq) {}
    #[inline]
    pub fn cross_reverse(&mut self, other: &mut Seq) {}
    fn cross_preamble<'a>(
        &'a mut self,
        other: &'a mut Seq,
    ) -> Option<(&'a mut Vec<Literal>, &'a mut Vec<Literal>)> {}
    #[inline]
    pub fn union(&mut self, other: &mut Seq) {}
    #[inline]
    pub fn union_into_empty(&mut self, other: &mut Seq) {}
    #[inline]
    pub fn dedup(&mut self) {}
    #[inline]
    pub fn sort(&mut self) {}
    #[inline]
    pub fn reverse_literals(&mut self) {}
    #[inline]
    pub fn minimize_by_preference(&mut self) {
        if let Some(ref mut lits) = self.literals {
            PreferenceTrie::minimize(lits, false);
        }
    }
    #[inline]
    pub fn keep_first_bytes(&mut self, len: usize) {}
    #[inline]
    pub fn keep_last_bytes(&mut self, len: usize) {}
    #[inline]
    pub fn is_finite(&self) -> bool {}
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn len(&self) -> Option<usize> {}
    #[inline]
    pub fn is_exact(&self) -> bool {}
    #[inline]
    pub fn is_inexact(&self) -> bool {}
    #[inline]
    pub fn max_union_len(&self, other: &Seq) -> Option<usize> {}
    #[inline]
    pub fn max_cross_len(&self, other: &Seq) -> Option<usize> {}
    #[inline]
    pub fn min_literal_len(&self) -> Option<usize> {}
    #[inline]
    pub fn max_literal_len(&self) -> Option<usize> {}
    #[inline]
    pub fn longest_common_prefix(&self) -> Option<&[u8]> {}
    #[inline]
    pub fn longest_common_suffix(&self) -> Option<&[u8]> {}
    #[inline]
    pub fn optimize_for_prefix_by_preference(&mut self) {}
    #[inline]
    pub fn optimize_for_suffix_by_preference(&mut self) {}
    fn optimize_by_preference(&mut self, prefix: bool) {}
}
impl PreferenceTrie {
    fn minimize(literals: &mut Vec<Literal>, keep_exact: bool) {
        let mut trie = PreferenceTrie {
            states: vec![],
            matches: vec![],
            next_literal_index: 1,
        };
        let mut make_inexact = vec![];
        literals
            .retain_mut(|lit| match trie.insert(lit.as_bytes()) {
                Ok(_) => true,
                Err(i) => {
                    if !keep_exact {
                        make_inexact.push(i.checked_sub(1).unwrap());
                    }
                    false
                }
            });
        for i in make_inexact {
            literals[i].make_inexact();
        }
    }
    fn insert(&mut self, bytes: &[u8]) -> Result<usize, usize> {}
    fn root(&mut self) -> usize {}
    fn create_state(&mut self) -> usize {}
}
