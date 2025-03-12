// Answer 0

#[test]
fn test_visit_pre_empty() {
    let mut writer = String::new();
    let hir = Hir {
        kind: HirKind::Empty,
        props: Properties::default(),
    };
    Writer { wtr: &mut writer }.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_unicode_class_empty() {
    let mut writer = String::new();
    let unicode_class = ClassUnicode::empty();
    let hir = Hir {
        kind: HirKind::Class(Class::Unicode(unicode_class)),
        props: Properties::default(),
    };
    Writer { wtr: &mut writer }.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_bytes_class_empty() {
    let mut writer = String::new();
    let bytes_class = ClassBytes::empty();
    let hir = Hir {
        kind: HirKind::Class(Class::Bytes(bytes_class)),
        props: Properties::default(),
    };
    Writer { wtr: &mut writer }.visit_pre(&hir).unwrap();
}

