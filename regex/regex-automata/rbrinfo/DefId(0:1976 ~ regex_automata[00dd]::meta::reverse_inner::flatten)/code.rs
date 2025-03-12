fn flatten(hir: &Hir) -> Hir {
    match hir.kind() {
        HirKind::Empty => Hir::empty(),
        HirKind::Literal(hir::Literal(ref x)) => Hir::literal(x.clone()),
        HirKind::Class(ref x) => Hir::class(x.clone()),
        HirKind::Look(ref x) => Hir::look(x.clone()),
        HirKind::Repetition(ref x) => Hir::repetition(x.with(flatten(&x.sub))),
        // This is the interesting case. We just drop the group information
        // entirely and use the child HIR itself.
        HirKind::Capture(hir::Capture { ref sub, .. }) => flatten(sub),
        HirKind::Alternation(ref xs) => {
            Hir::alternation(xs.iter().map(|x| flatten(x)).collect())
        }
        HirKind::Concat(ref xs) => {
            Hir::concat(xs.iter().map(|x| flatten(x)).collect())
        }
    }
}