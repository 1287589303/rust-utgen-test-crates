use alloc::vec::Vec;
use regex_syntax::hir::{self, literal, Hir, HirKind};
use crate::{util::prefilter::Prefilter, MatchKind};
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
