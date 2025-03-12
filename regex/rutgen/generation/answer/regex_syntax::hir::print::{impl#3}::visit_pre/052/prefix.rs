// Answer 0

#[test]
fn test_visit_pre_literal() {
    let bytes: Vec<u8> = vec![b'a', b'b', b'c'];
    let hir = Hir {
        kind: HirKind::Literal(hir::Literal(bytes.clone())),
        props: Properties::default(),
    };
    let mut buffer = String::new();
    let mut writer = Writer { wtr: &mut buffer };
    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_class_bytes() {
    let start = 10;
    let end = 11;
    let range = ClassBytesRange::new(start, end);
    let class_bytes = ClassBytes::new(vec![range]);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Bytes(class_bytes)),
        props: Properties::default(),
    };
    let mut buffer = String::new();
    let mut writer = Writer { wtr: &mut buffer };
    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_class_unicode() {
    let range_start = 'a';
    let range_end = 'b';
    let range = ClassUnicodeRange::new(range_start, range_end);
    let class_unicode = ClassUnicode::new(vec![range]);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(class_unicode)),
        props: Properties::default(),
    };
    let mut buffer = String::new();
    let mut writer = Writer { wtr: &mut buffer };
    let _ = writer.visit_pre(&hir);
}

