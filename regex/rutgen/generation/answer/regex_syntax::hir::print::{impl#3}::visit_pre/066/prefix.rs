// Answer 0

#[test]
fn test_visit_pre_empty_hir_kind() {
    let mut writer = Vec::new();
    let mut visitor = Writer { wtr: &mut writer };
    let hir = Hir {
        kind: HirKind::Empty,
        props: Properties::default(),
    };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_unicode_class_empty_ranges() {
    let mut writer = Vec::new();
    let mut visitor = Writer { wtr: &mut writer };
    let cls = ClassUnicode::new(vec![
        ClassUnicodeRange::new('a', 'a'),
        ClassUnicodeRange::new('b', 'b'),
    ]);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(cls)),
        props: Properties::default(),
    };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_unicode_class_non_empty_ranges() {
    let mut writer = Vec::new();
    let mut visitor = Writer { wtr: &mut writer };
    let cls = ClassUnicode::new(vec![
        ClassUnicodeRange::new('a', 'a'),
    ]);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(cls)),
        props: Properties::default(),
    };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_bytes_class_single_range() {
    let mut writer = Vec::new();
    let mut visitor = Writer { wtr: &mut writer };
    let cls = ClassBytes::new(vec![
        ClassBytesRange::new(65, 65), // A
    ]);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Bytes(cls)),
        props: Properties::default(),
    };
    let _ = visitor.visit_pre(&hir);
}

#[test]
fn test_visit_pre_bytes_class_multiple_ranges() {
    let mut writer = Vec::new();
    let mut visitor = Writer { wtr: &mut writer };
    let cls = ClassBytes::new(vec![
        ClassBytesRange::new(65, 90), // A-Z
        ClassBytesRange::new(97, 122), // a-z
    ]);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Bytes(cls)),
        props: Properties::default(),
    };
    let _ = visitor.visit_pre(&hir);
}

