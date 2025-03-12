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
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Literal {
    bytes: Vec<u8>,
    exact: bool,
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
    pub fn cross_reverse(&mut self, other: &mut Seq) {
        let (lits1, lits2) = match self.cross_preamble(other) {
            None => return,
            Some((lits1, lits2)) => (lits1, lits2),
        };
        let newcap = lits1.len().saturating_mul(lits2.len());
        let selflits = mem::replace(lits1, Vec::with_capacity(newcap));
        for (i, otherlit) in lits2.drain(..).enumerate() {
            for selflit in selflits.iter() {
                if !selflit.is_exact() {
                    if i == 0 {
                        lits1.push(selflit.clone());
                    }
                    continue;
                }
                let mut newlit = Literal::exact(
                    Vec::with_capacity(otherlit.len() + selflit.len()),
                );
                newlit.extend(&otherlit);
                newlit.extend(&selflit);
                if !otherlit.is_exact() {
                    newlit.make_inexact();
                }
                lits1.push(newlit);
            }
        }
        self.dedup();
    }
    fn cross_preamble<'a>(
        &'a mut self,
        other: &'a mut Seq,
    ) -> Option<(&'a mut Vec<Literal>, &'a mut Vec<Literal>)> {
        let lits2 = match other.literals {
            None => {
                if self.min_literal_len() == Some(0) {
                    *self = Seq::infinite();
                } else {
                    self.make_inexact();
                }
                return None;
            }
            Some(ref mut lits) => lits,
        };
        let lits1 = match self.literals {
            None => {
                lits2.drain(..);
                return None;
            }
            Some(ref mut lits) => lits,
        };
        Some((lits1, lits2))
    }
    #[inline]
    pub fn union(&mut self, other: &mut Seq) {}
    #[inline]
    pub fn union_into_empty(&mut self, other: &mut Seq) {}
    #[inline]
    pub fn dedup(&mut self) {
        if let Some(ref mut lits) = self.literals {
            lits.dedup_by(|lit1, lit2| {
                if lit1.as_bytes() != lit2.as_bytes() {
                    return false;
                }
                if lit1.is_exact() != lit2.is_exact() {
                    lit1.make_inexact();
                    lit2.make_inexact();
                }
                true
            });
        }
    }
    #[inline]
    pub fn sort(&mut self) {}
    #[inline]
    pub fn reverse_literals(&mut self) {}
    #[inline]
    pub fn minimize_by_preference(&mut self) {}
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
impl Literal {
    #[inline]
    pub fn exact<B: Into<Vec<u8>>>(bytes: B) -> Literal {}
    #[inline]
    pub fn inexact<B: Into<Vec<u8>>>(bytes: B) -> Literal {}
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {}
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {}
    #[inline]
    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn is_exact(&self) -> bool {
        self.exact
    }
    #[inline]
    pub fn make_inexact(&mut self) {
        self.exact = false;
    }
    #[inline]
    pub fn reverse(&mut self) {}
    #[inline]
    pub fn extend(&mut self, lit: &Literal) {
        if !self.is_exact() {
            return;
        }
        self.bytes.extend_from_slice(&lit.bytes);
    }
    #[inline]
    pub fn keep_first_bytes(&mut self, len: usize) {}
    #[inline]
    pub fn keep_last_bytes(&mut self, len: usize) {}
    fn is_poisonous(&self) -> bool {}
}
