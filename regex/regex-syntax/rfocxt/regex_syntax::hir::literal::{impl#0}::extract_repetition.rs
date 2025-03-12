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
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Literal {
    bytes: Vec<u8>,
    exact: bool,
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
    pub fn extract(&self, hir: &Hir) -> Seq {
        use crate::hir::HirKind::*;
        match *hir.kind() {
            Empty | Look(_) => Seq::singleton(self::Literal::exact(vec![])),
            Literal(hir::Literal(ref bytes)) => {
                let mut seq = Seq::singleton(self::Literal::exact(bytes.to_vec()));
                self.enforce_literal_len(&mut seq);
                seq
            }
            Class(hir::Class::Unicode(ref cls)) => self.extract_class_unicode(cls),
            Class(hir::Class::Bytes(ref cls)) => self.extract_class_bytes(cls),
            Repetition(ref rep) => self.extract_repetition(rep),
            Capture(hir::Capture { ref sub, .. }) => self.extract(sub),
            Concat(ref hirs) => {
                match self.kind {
                    ExtractKind::Prefix => self.extract_concat(hirs.iter()),
                    ExtractKind::Suffix => self.extract_concat(hirs.iter().rev()),
                }
            }
            Alternation(ref hirs) => self.extract_alternation(hirs.iter()),
        }
    }
    pub fn kind(&mut self, kind: ExtractKind) -> &mut Extractor {}
    pub fn limit_class(&mut self, limit: usize) -> &mut Extractor {}
    pub fn limit_repeat(&mut self, limit: usize) -> &mut Extractor {}
    pub fn limit_literal_len(&mut self, limit: usize) -> &mut Extractor {}
    pub fn limit_total(&mut self, limit: usize) -> &mut Extractor {}
    fn extract_concat<'a, I: Iterator<Item = &'a Hir>>(&self, it: I) -> Seq {}
    fn extract_alternation<'a, I: Iterator<Item = &'a Hir>>(&self, it: I) -> Seq {}
    fn extract_repetition(&self, rep: &hir::Repetition) -> Seq {
        let mut subseq = self.extract(&rep.sub);
        match *rep {
            hir::Repetition { min: 0, max, greedy, .. } => {
                if max != Some(1) {
                    subseq.make_inexact();
                }
                let mut empty = Seq::singleton(Literal::exact(vec![]));
                if !greedy {
                    mem::swap(&mut subseq, &mut empty);
                }
                self.union(subseq, &mut empty)
            }
            hir::Repetition { min, max: Some(max), .. } if min == max => {
                assert!(min > 0);
                let limit = u32::try_from(self.limit_repeat).unwrap_or(u32::MAX);
                let mut seq = Seq::singleton(Literal::exact(vec![]));
                for _ in 0..cmp::min(min, limit) {
                    if seq.is_inexact() {
                        break;
                    }
                    seq = self.cross(seq, &mut subseq.clone());
                }
                if usize::try_from(min).is_err() || min > limit {
                    seq.make_inexact();
                }
                seq
            }
            hir::Repetition { min, .. } => {
                assert!(min > 0);
                let limit = u32::try_from(self.limit_repeat).unwrap_or(u32::MAX);
                let mut seq = Seq::singleton(Literal::exact(vec![]));
                for _ in 0..cmp::min(min, limit) {
                    if seq.is_inexact() {
                        break;
                    }
                    seq = self.cross(seq, &mut subseq.clone());
                }
                seq.make_inexact();
                seq
            }
        }
    }
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
    fn union(&self, mut seq1: Seq, seq2: &mut Seq) -> Seq {
        if seq1.max_union_len(seq2).map_or(false, |len| len > self.limit_total) {
            match self.kind {
                ExtractKind::Prefix => {
                    seq1.keep_first_bytes(4);
                    seq2.keep_first_bytes(4);
                }
                ExtractKind::Suffix => {
                    seq1.keep_last_bytes(4);
                    seq2.keep_last_bytes(4);
                }
            }
            seq1.dedup();
            seq2.dedup();
            if seq1.max_union_len(seq2).map_or(false, |len| len > self.limit_total) {
                seq2.make_infinite();
            }
        }
        seq1.union(seq2);
        assert!(seq1.len().map_or(true, | x | x <= self.limit_total));
        seq1
    }
    fn enforce_literal_len(&self, seq: &mut Seq) {}
}
impl Seq {
    #[inline]
    pub fn empty() -> Seq {}
    #[inline]
    pub fn infinite() -> Seq {}
    #[inline]
    pub fn singleton(lit: Literal) -> Seq {
        Seq { literals: Some(vec![lit]) }
    }
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
    pub fn make_inexact(&mut self) {
        let lits = match self.literals {
            None => return,
            Some(ref mut lits) => lits,
        };
        for lit in lits.iter_mut() {
            lit.make_inexact();
        }
    }
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
    pub fn is_inexact(&self) -> bool {
        self.literals().map_or(true, |lits| lits.iter().all(|x| !x.is_exact()))
    }
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
