// Answer 0

#[test]
fn test_visit_pre_empty() {
    let mut writer = String::new();
    let mut visitor = Writer { wtr: &mut writer };
    let hir = Hir {
        kind: HirKind::Empty,
        props: Properties::default(),
    };
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_unicode_class_non_empty() {
    let mut writer = String::new();
    let mut visitor = Writer { wtr: &mut writer };
    let unicode_class = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'b')]);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(unicode_class)),
        props: Properties::default(),
    };
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_bytes_class_non_empty() {
    let mut writer = String::new();
    let mut visitor = Writer { wtr: &mut writer };
    let bytes_class = ClassBytes::new(vec![ClassBytesRange::new(1, 2)]);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Bytes(bytes_class)),
        props: Properties::default(),
    };
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_unicode_class_ranges_non_empty() {
    let mut writer = String::new();
    let mut visitor = Writer { wtr: &mut writer };
    let unicode_class = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'c')]);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(unicode_class)),
        props: Properties::default(),
    };
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_bytes_class_ranges_non_empty() {
    let mut writer = String::new();
    let mut visitor = Writer { wtr: &mut writer };
    let bytes_class = ClassBytes::new(vec![ClassBytesRange::new(1, 3)]);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Bytes(bytes_class)),
        props: Properties::default(),
    };
    visitor.visit_pre(&hir).unwrap();
}

