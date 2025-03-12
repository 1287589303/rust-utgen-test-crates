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
    pub fn limit_repeat(&mut self, limit: usize) -> &mut Extractor {
        self.limit_repeat = limit;
        self
    }
    pub fn limit_literal_len(&mut self, limit: usize) -> &mut Extractor {}
    pub fn limit_total(&mut self, limit: usize) -> &mut Extractor {}
    fn extract_concat<'a, I: Iterator<Item = &'a Hir>>(&self, it: I) -> Seq {}
    fn extract_alternation<'a, I: Iterator<Item = &'a Hir>>(&self, it: I) -> Seq {}
    fn extract_repetition(&self, rep: &hir::Repetition) -> Seq {}
    fn extract_class_unicode(&self, cls: &hir::ClassUnicode) -> Seq {}
    fn extract_class_bytes(&self, cls: &hir::ClassBytes) -> Seq {}
    fn class_over_limit_unicode(&self, cls: &hir::ClassUnicode) -> bool {}
    fn class_over_limit_bytes(&self, cls: &hir::ClassBytes) -> bool {}
    fn cross(&self, mut seq1: Seq, seq2: &mut Seq) -> Seq {}
    fn union(&self, mut seq1: Seq, seq2: &mut Seq) -> Seq {}
    fn enforce_literal_len(&self, seq: &mut Seq) {}
}
