// Answer 0

#[test]
fn test_class_bytes_union_with_unicode_and_bytes() {
    let unicode_class = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange { start: 'a' as u32, end: 'z' as u32 }]));
    let byte_class = Class::Bytes(ClassBytes::new(vec![ClassBytesRange { start: 0, end: 127 }]));
    let hirs = vec![
        Hir { kind: HirKind::Class(unicode_class.clone()), props: Properties::default() },
        Hir { kind: HirKind::Class(byte_class.clone()), props: Properties::default() },
    ];
    let _result = class_bytes(&hirs);
}

#[test]
fn test_class_bytes_union_with_multiple_unicode_and_bytes() {
    let unicode_class1 = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange { start: 'A' as u32, end: 'Z' as u32 }]));
    let unicode_class2 = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange { start: 'a' as u32, end: 'z' as u32 }]));
    let byte_class1 = Class::Bytes(ClassBytes::new(vec![ClassBytesRange { start: 0, end: 63 }]));
    let byte_class2 = Class::Bytes(ClassBytes::new(vec![ClassBytesRange { start: 64, end: 127 }]));
    let hirs = vec![
        Hir { kind: HirKind::Class(unicode_class1.clone()), props: Properties::default() },
        Hir { kind: HirKind::Class(unicode_class2.clone()), props: Properties::default() },
        Hir { kind: HirKind::Class(byte_class1.clone()), props: Properties::default() },
        Hir { kind: HirKind::Class(byte_class2.clone()), props: Properties::default() },
    ];
    let _result = class_bytes(&hirs);
}

#[test]
fn test_class_bytes_union_with_only_unicode() {
    let unicode_class = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange { start: '0' as u32, end: '9' as u32 }]));
    let hirs = vec![
        Hir { kind: HirKind::Class(unicode_class.clone()), props: Properties::default() },
    ];
    let _result = class_bytes(&hirs);
}

#[test]
fn test_class_bytes_union_with_only_bytes() {
    let byte_class = Class::Bytes(ClassBytes::new(vec![ClassBytesRange { start: 200, end: 255 }]));
    let hirs = vec![
        Hir { kind: HirKind::Class(byte_class.clone()), props: Properties::default() },
    ];
    let _result = class_bytes(&hirs);
}

#[test]
#[should_panic]
fn test_class_bytes_invalid_hir_kind() {
    let hirs = vec![
        Hir { kind: HirKind::Empty, props: Properties::default() },
    ];
    let _result = class_bytes(&hirs);
}

