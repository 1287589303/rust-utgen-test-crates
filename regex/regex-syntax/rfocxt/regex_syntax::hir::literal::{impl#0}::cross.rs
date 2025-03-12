use core::{cmp, mem, num::NonZeroUsize};
use alloc::{vec, vec::Vec};
use crate::hir::{self, Hir};
#[derive(Clone, Debug)]
pub struct Extractor {
    kind: ExtractKind,
    limit_class: usize,
    limit_repeat: usize,
    limit_literal_len: usize,
    limit_total: usize,
}
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
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum ExtractKind {
    /// Extracts only prefix literals from a regex.
    Prefix,
    /// Extracts only suffix literals from a regex.
    ///
    /// Note that the sequence returned by suffix literals currently may
    /// not correctly represent leftmost-first or "preference" order match
    /// semantics.
    Suffix,
}
impl Extractor {
    pub fn new() -> Extractor {}
    pub fn extract(&self, hir: &Hir) -> Seq {}
    pub fn kind(&mut self, kind: ExtractKind) -> &mut Extractor {}
    pub fn limit_class(&mut self, limit: usize) -> &mut Extractor {}
    pub fn limit_repeat(&mut self, limit: usize) -> &mut Extractor {}
    pub fn limit_literal_len(&mut self, limit: usize) -> &mut Extractor {}
    pub fn limit_total(&mut self, limit: usize) -> &mut Extractor {}
    fn extract_concat<'a, I: Iterator<Item = &'a Hir>>(&self, it: I) -> Seq {}
    fn extract_alternation<'a, I: Iterator<Item = &'a Hir>>(&self, it: I) -> Seq {}
    fn extract_repetition(&self, rep: &hir::Repetition) -> Seq {}
    fn extract_class_unicode(&self, cls: &hir::ClassUnicode) -> Seq {}
    fn extract_class_bytes(&self, cls: &hir::ClassBytes) -> Seq {}
    fn class_over_limit_unicode(&self, cls: &hir::ClassUnicode) -> bool {}
    fn class_over_limit_bytes(&self, cls: &hir::ClassBytes) -> bool {}
    fn cross(&self, mut seq1: Seq, seq2: &mut Seq) -> Seq {
        if seq1.max_cross_len(seq2).map_or(false, |len| len > self.limit_total) {
            seq2.make_infinite();
        }
        if let ExtractKind::Suffix = self.kind {
            seq1.cross_reverse(seq2);
        } else {
            seq1.cross_forward(seq2);
        }
        assert!(seq1.len().map_or(true, | x | x <= self.limit_total));
        self.enforce_literal_len(&mut seq1);
        seq1
    }
    fn union(&self, mut seq1: Seq, seq2: &mut Seq) -> Seq {}
    fn enforce_literal_len(&self, seq: &mut Seq) {
        let len = self.limit_literal_len;
        match self.kind {
            ExtractKind::Prefix => seq.keep_first_bytes(len),
            ExtractKind::Suffix => seq.keep_last_bytes(len),
        }
    }
}
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
    pub fn make_infinite(&mut self) {
        self.literals = None;
    }
    #[inline]
    pub fn cross_forward(&mut self, other: &mut Seq) {
        let (lits1, lits2) = match self.cross_preamble(other) {
            None => return,
            Some((lits1, lits2)) => (lits1, lits2),
        };
        let newcap = lits1.len().saturating_mul(lits2.len());
        for selflit in mem::replace(lits1, Vec::with_capacity(newcap)) {
            if !selflit.is_exact() {
                lits1.push(selflit);
                continue;
            }
            for otherlit in lits2.iter() {
                let mut newlit = Literal::exact(
                    Vec::with_capacity(selflit.len() + otherlit.len()),
                );
                newlit.extend(&selflit);
                newlit.extend(&otherlit);
                if !otherlit.is_exact() {
                    newlit.make_inexact();
                }
                lits1.push(newlit);
            }
        }
        lits2.drain(..);
        self.dedup();
    }
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
    pub fn len(&self) -> Option<usize> {
        self.literals.as_ref().map(|lits| lits.len())
    }
    #[inline]
    pub fn is_exact(&self) -> bool {}
    #[inline]
    pub fn is_inexact(&self) -> bool {}
    #[inline]
    pub fn max_union_len(&self, other: &Seq) -> Option<usize> {}
    #[inline]
    pub fn max_cross_len(&self, other: &Seq) -> Option<usize> {
        let len1 = self.len()?;
        let len2 = other.len()?;
        Some(len1.saturating_mul(len2))
    }
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
