// Answer 0

#[test]
fn test_extract_literal() {
    let extractor = Extractor::new();

    let bytes = vec![65, 66, 67]; // Represents the string "ABC"
    let hir = Hir { 
        kind: HirKind::Literal(hir::Literal(bytes.clone())), 
        props: Properties::default() 
    };

    let result = extractor.extract(&hir);
}

#[test]
fn test_extract_class_bytes() {
    let extractor = Extractor::new();

    let cls = hir::Class::Bytes(ClassBytes { 
        set: IntervalSet::new(vec![ClassBytesRange::new(65, 90)]) // Represents "A-Z"
    });
    let hir = Hir {
        kind: HirKind::Class(cls),
        props: Properties::default()
    };

    let result = extractor.extract(&hir);
}

#[test]
fn test_extract_class_unicode() {
    let extractor = Extractor::new();

    let cls = hir::Class::Unicode(ClassUnicode {
        set: IntervalSet::new(vec![ClassUnicodeRange::new(192, 255)]) // Represents valid Unicode range
    });
    let hir = Hir {
        kind: HirKind::Class(cls),
        props: Properties::default()
    };

    let result = extractor.extract(&hir);
}

