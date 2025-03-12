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
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClassUnicodeRange {
    start: char,
    end: char,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
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
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Literal {
    bytes: Vec<u8>,
    exact: bool,
}
#[derive(Debug)]
pub struct ClassUnicodeIter<'a>(IntervalSetIter<'a, ClassUnicodeRange>);
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
    fn extract_class_unicode(&self, cls: &hir::ClassUnicode) -> Seq {
        if self.class_over_limit_unicode(cls) {
            return Seq::infinite();
        }
        let mut seq = Seq::empty();
        for r in cls.iter() {
            for ch in r.start()..=r.end() {
                seq.push(Literal::from(ch));
            }
        }
        self.enforce_literal_len(&mut seq);
        seq
    }
    fn extract_class_bytes(&self, cls: &hir::ClassBytes) -> Seq {}
    fn class_over_limit_unicode(&self, cls: &hir::ClassUnicode) -> bool {
        let mut count = 0;
        for r in cls.iter() {
            if count > self.limit_class {
                return true;
            }
            count += r.len();
        }
        count > self.limit_class
    }
    fn class_over_limit_bytes(&self, cls: &hir::ClassBytes) -> bool {}
    fn cross(&self, mut seq1: Seq, seq2: &mut Seq) -> Seq {}
    fn union(&self, mut seq1: Seq, seq2: &mut Seq) -> Seq {}
    fn enforce_literal_len(&self, seq: &mut Seq) {
        let len = self.limit_literal_len;
        match self.kind {
            ExtractKind::Prefix => seq.keep_first_bytes(len),
            ExtractKind::Suffix => seq.keep_last_bytes(len),
        }
    }
}
impl ClassUnicodeRange {
    pub fn new(start: char, end: char) -> ClassUnicodeRange {}
    pub fn start(&self) -> char {
        self.start
    }
    pub fn end(&self) -> char {
        self.end
    }
    pub fn len(&self) -> usize {}
}
impl ClassUnicode {
    pub fn new<I>(ranges: I) -> ClassUnicode
    where
        I: IntoIterator<Item = ClassUnicodeRange>,
    {}
    pub fn empty() -> ClassUnicode {}
    pub fn push(&mut self, range: ClassUnicodeRange) {}
    pub fn iter(&self) -> ClassUnicodeIter<'_> {
        ClassUnicodeIter(self.set.iter())
    }
    pub fn ranges(&self) -> &[ClassUnicodeRange] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn try_case_fold_simple(&mut self) -> core::result::Result<(), CaseFoldError> {}
    pub fn negate(&mut self) {}
    pub fn union(&mut self, other: &ClassUnicode) {}
    pub fn intersect(&mut self, other: &ClassUnicode) {}
    pub fn difference(&mut self, other: &ClassUnicode) {}
    pub fn symmetric_difference(&mut self, other: &ClassUnicode) {}
    pub fn is_ascii(&self) -> bool {}
    pub fn minimum_len(&self) -> Option<usize> {}
    pub fn maximum_len(&self) -> Option<usize> {}
    pub fn literal(&self) -> Option<Vec<u8>> {}
    pub fn to_byte_class(&self) -> Option<ClassBytes> {}
}
impl Seq {
    #[inline]
    pub fn empty() -> Seq {
        Seq { literals: Some(vec![]) }
    }
    #[inline]
    pub fn infinite() -> Seq {
        Seq { literals: None }
    }
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
    pub fn push(&mut self, lit: Literal) {
        let lits = match self.literals {
            None => return,
            Some(ref mut lits) => lits,
        };
        if lits.last().map_or(false, |m| m == &lit) {
            return;
        }
        lits.push(lit);
    }
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
