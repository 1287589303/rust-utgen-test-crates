use alloc::vec::Vec;
use regex_syntax::hir::{self, literal, Hir, HirKind};
use crate::{util::prefilter::Prefilter, MatchKind};
#[derive(Clone, Debug)]
pub struct Prefilter {
    #[cfg(not(feature = "alloc"))]
    _unused: (),
    #[cfg(feature = "alloc")]
    pre: Arc<dyn PrefilterI>,
    #[cfg(feature = "alloc")]
    is_fast: bool,
    #[cfg(feature = "alloc")]
    max_needle_len: usize,
}
impl Prefilter {
    pub fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Prefilter> {}
    fn from_choice(choice: Choice, max_needle_len: usize) -> Option<Prefilter> {}
    #[cfg(feature = "syntax")]
    pub fn from_hir_prefix(kind: MatchKind, hir: &Hir) -> Option<Prefilter> {}
    #[cfg(feature = "syntax")]
    pub fn from_hirs_prefix<H: Borrow<Hir>>(
        kind: MatchKind,
        hirs: &[H],
    ) -> Option<Prefilter> {}
    #[inline]
    pub fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    #[inline]
    pub fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
    #[inline]
    pub fn max_needle_len(&self) -> usize {}
    #[inline]
    pub fn is_fast(&self) -> bool {
        #[cfg(not(feature = "alloc"))] { unreachable!() }
        #[cfg(feature = "alloc")] { self.is_fast }
    }
}
pub(crate) fn extract(hirs: &[&Hir]) -> Option<(Hir, Prefilter)> {
    if hirs.len() != 1 {
        debug!(
            "skipping reverse inner optimization since it only \
		 	 supports 1 pattern, {} were given",
            hirs.len(),
        );
        return None;
    }
    let mut concat = match top_concat(hirs[0]) {
        Some(concat) => concat,
        None => {
            debug!(
                "skipping reverse inner optimization because a top-level \
		 	     concatenation could not found",
            );
            return None;
        }
    };
    for i in 1..concat.len() {
        let hir = &concat[i];
        let pre = match prefilter(hir) {
            None => continue,
            Some(pre) => pre,
        };
        if !pre.is_fast() {
            debug!(
                "skipping extracted inner prefilter because \
				 it probably isn't fast"
            );
            continue;
        }
        let concat_suffix = Hir::concat(concat.split_off(i));
        let concat_prefix = Hir::concat(concat);
        let pre2 = match prefilter(&concat_suffix) {
            None => pre,
            Some(pre2) => if pre2.is_fast() { pre2 } else { pre }
        };
        return Some((concat_prefix, pre2));
    }
    debug!(
        "skipping reverse inner optimization because a top-level \
	     sub-expression with a fast prefilter could not be found"
    );
    None
}
fn prefilter(hir: &Hir) -> Option<Prefilter> {
    let mut extractor = literal::Extractor::new();
    extractor.kind(literal::ExtractKind::Prefix);
    let mut prefixes = extractor.extract(hir);
    debug!(
        "inner prefixes (len={:?}) extracted before optimization: {:?}", prefixes.len(),
        prefixes
    );
    prefixes.make_inexact();
    prefixes.optimize_for_prefix_by_preference();
    debug!(
        "inner prefixes (len={:?}) extracted after optimization: {:?}", prefixes.len(),
        prefixes
    );
    prefixes.literals().and_then(|lits| Prefilter::new(MatchKind::LeftmostFirst, lits))
}
fn top_concat(mut hir: &Hir) -> Option<Vec<Hir>> {
    loop {
        hir = match hir.kind() {
            HirKind::Empty
            | HirKind::Literal(_)
            | HirKind::Class(_)
            | HirKind::Look(_)
            | HirKind::Repetition(_)
            | HirKind::Alternation(_) => return None,
            HirKind::Capture(hir::Capture { ref sub, .. }) => sub,
            HirKind::Concat(ref subs) => {
                let concat = Hir::concat(subs.iter().map(|h| flatten(h)).collect());
                return match concat.into_kind() {
                    HirKind::Concat(xs) => Some(xs),
                    _ => return None,
                };
            }
        };
    }
}
