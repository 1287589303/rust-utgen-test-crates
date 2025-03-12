// Answer 0

#[test]
fn test_class_chars_unicode_class() {
    let unicode_range = ClassUnicodeRange { start: 'a', end: 'z' };
    let class_unicode = ClassUnicode::new(vec![unicode_range]);
    let hir_unicode = Hir {
        kind: HirKind::Class(Class::Unicode(class_unicode)),
        props: Properties::default(),
    };
    let hirs = vec![hir_unicode];
    let _ = class_chars(&hirs);
}

#[test]
fn test_class_chars_bytes_class_none() {
    let class_bytes = ClassBytes::empty();
    let hir_bytes = Hir {
        kind: HirKind::Class(Class::Bytes(class_bytes)),
        props: Properties::default(),
    };
    let hirs = vec![hir_bytes];
    let _ = class_chars(&hirs);
}

#[test]
fn test_class_chars_multiple_classes() {
    let unicode_range1 = ClassUnicodeRange { start: 'a', end: 'f' };
    let class_unicode1 = ClassUnicode::new(vec![unicode_range1]);

    let unicode_range2 = ClassUnicodeRange { start: 'g', end: 'z' };
    let class_unicode2 = ClassUnicode::new(vec![unicode_range2]);

    let hir_unicode1 = Hir {
        kind: HirKind::Class(Class::Unicode(class_unicode1)),
        props: Properties::default(),
    };
    let hir_unicode2 = Hir {
        kind: HirKind::Class(Class::Unicode(class_unicode2)),
        props: Properties::default(),
    };

    let hirs = vec![hir_unicode1, hir_unicode2];
    let _ = class_chars(&hirs);
}

#[test]
fn test_class_chars_bytes_class_to_unicode_class_none() {
    let class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 256, end: 255 }]);
    let hir_bytes = Hir {
        kind: HirKind::Class(Class::Bytes(class_bytes)),
        props: Properties::default(),
    };
    let hirs = vec![hir_bytes];
    let _ = class_chars(&hirs);
}

