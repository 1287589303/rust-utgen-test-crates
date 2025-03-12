// Answer 0

#[test]
fn test_extract_empty() {
    let extractor = Extractor::new();
    let hir = Hir { kind: HirKind::Empty, props: Properties::default() };
    let result = extractor.extract(&hir);
}

#[test]
fn test_extract_look_around() {
    let extractor = Extractor::new();
    let hir = Hir { kind: HirKind::Look(Look::new()), props: Properties::default() };
    let result = extractor.extract(&hir);
}

