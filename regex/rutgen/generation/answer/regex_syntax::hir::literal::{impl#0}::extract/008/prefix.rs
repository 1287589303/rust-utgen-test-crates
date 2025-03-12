// Answer 0

#[test]
fn test_extract_empty() {
    let extractor = Extractor::new();
    let hir = Hir {
        kind: HirKind::Empty,
        props: Properties::default(),
    };
    let result = extractor.extract(&hir);
}

#[test]
fn test_extract_class_bytes() {
    let extractor = Extractor::new();
    let bytes_range = IntervalSet::new(vec![ClassBytesRange::new(0u8..=255u8)]);
    let cls = ClassBytes { set: bytes_range };
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Bytes(cls)),
        props: Properties::default(),
    };
    let result = extractor.extract(&hir);
}

#[test]
fn test_extract_class_unicode() {
    let extractor = Extractor::new();
    let unicode_range = IntervalSet::new(vec![ClassUnicodeRange::new('a'..='z')]);
    let cls = ClassUnicode { set: unicode_range };
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(cls)),
        props: Properties::default(),
    };
    let result = extractor.extract(&hir);
}

