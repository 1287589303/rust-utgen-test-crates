// Answer 0

#[test]
fn test_visit_pre_literal() {
    let mut output = String::new();
    let mut writer = Writer { wtr: &mut output };

    let bytes_range = ClassBytesRange::new(10, 20);
    let class_bytes = ClassBytes::new(vec![bytes_range]);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Bytes(class_bytes)),
        props: Default::default(),
    };

    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_unicode() {
    let mut output = String::new();
    let mut writer = Writer { wtr: &mut output };

    let unicode_range = ClassUnicodeRange::new('a', 'c');
    let class_unicode = ClassUnicode::new(vec![unicode_range]);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(class_unicode)),
        props: Default::default(),
    };

    let _ = writer.visit_pre(&hir);
}

