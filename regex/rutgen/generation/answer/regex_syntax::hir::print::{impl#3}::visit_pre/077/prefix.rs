// Answer 0

#[test]
fn test_visit_pre_empty_hir() {
    let mut output = String::new();
    let writer = Writer { wtr: &mut output };
    let hir = Hir {
        kind: HirKind::Empty,
        props: Properties::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_unicode_non_empty_ranges() {
    let mut output = String::new();
    let mut unicode_class = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'z')]);
    let writer = Writer { wtr: &mut output };
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(unicode_class.clone())),
        props: Properties::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_bytes_non_empty_ranges() {
    let mut output = String::new();
    let mut byte_class = ClassBytes::new(vec![ClassBytesRange::new(1, 5)]);
    let writer = Writer { wtr: &mut output };
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Bytes(byte_class.clone())),
        props: Properties::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_unicode_class_empty() {
    let mut output = String::new();
    let unicode_class = ClassUnicode::empty();
    let writer = Writer { wtr: &mut output };
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(unicode_class)),
        props: Properties::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

