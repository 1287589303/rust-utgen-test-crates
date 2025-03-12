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
                // We are careful to only do the flattening/copy when we know
                // we have a "top level" concat we can inspect. This avoids
                // doing extra work in cases where we definitely won't use it.
                // (This might still be wasted work if we can't go on to find
                // some literals to extract.)
                let concat =
                    Hir::concat(subs.iter().map(|h| flatten(h)).collect());
                return match concat.into_kind() {
                    HirKind::Concat(xs) => Some(xs),
                    // It is actually possible for this case to occur, because
                    // 'Hir::concat' might simplify the expression to the point
                    // that concatenations are actually removed. One wonders
                    // whether this leads to other cases where we should be
                    // extracting literals, but in theory, I believe if we do
                    // get here, then it means that a "real" prefilter failed
                    // to be extracted and we should probably leave well enough
                    // alone. (A "real" prefilter is unbothered by "top-level
                    // concats" and "capturing groups.")
                    _ => return None,
                };
            }
        };
    }
}