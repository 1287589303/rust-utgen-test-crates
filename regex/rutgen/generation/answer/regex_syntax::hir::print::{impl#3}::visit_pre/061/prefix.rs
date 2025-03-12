// Answer 0

#[test]
fn test_visit_pre_literal() {
    let mut output = String::new();
    let mut writer = Writer { wtr: &mut output };
    let bytes = b"hello";
    let hir = Hir {
        kind: HirKind::Literal(hir::Literal(bytes.to_vec())),
        props: Properties::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_unicode() {
    let mut output = String::new();
    let mut writer = Writer { wtr: &mut output };
    let ranges = vec![ClassUnicodeRange::new('a', 'z')];
    let cls = ClassUnicode::new(ranges);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(cls)),
        props: Properties::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_bytes() {
    let mut output = String::new();
    let mut writer = Writer { wtr: &mut output };
    let ranges = vec![ClassBytesRange::new(1, 5)];
    let cls = ClassBytes::new(ranges);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Bytes(cls)),
        props: Properties::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

