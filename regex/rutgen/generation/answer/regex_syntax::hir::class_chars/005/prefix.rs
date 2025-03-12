// Answer 0

#[test]
fn test_class_chars_unicode_class() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new_unicode_class(ranges: Vec<ClassUnicodeRange>) -> Hir {
            let cls_unicode = ClassUnicode::new(ranges);
            Hir {
                kind: HirKind::Class(Class::Unicode(cls_unicode)),
                props: Properties::default(),
            }
        }

        fn new_bytes_class(ranges: Vec<ClassBytesRange>) -> Hir {
            let cls_bytes = ClassBytes::new(ranges);
            Hir {
                kind: HirKind::Class(Class::Bytes(cls_bytes)),
                props: Properties::default(),
            }
        }
    }

    let hirs: Vec<Hir> = vec![
        TestHir::new_unicode_class(vec![ClassUnicodeRange { start: 'a', end: 'z' }]),
        TestHir::new_bytes_class(vec![ClassBytesRange { start: 0x61, end: 0x7A }]), // ASCII for 'a' to 'z'
    ];

    let result = class_chars(&hirs);
}

#[test]
fn test_class_chars_bytes_class_converts_to_unicode() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new_bytes_class(ranges: Vec<ClassBytesRange>) -> Hir {
            let cls_bytes = ClassBytes::new(ranges);
            Hir {
                kind: HirKind::Class(Class::Bytes(cls_bytes)),
                props: Properties::default(),
            }
        }
    }

    let hirs: Vec<Hir> = vec![
        TestHir::new_bytes_class(vec![ClassBytesRange { start: 0x20, end: 0x7E }]), // ASCII printable characters
        TestHir::new_unicode_class(vec![ClassUnicodeRange { start: '0', end: '9' }]), // Unicode range
    ];

    let result = class_chars(&hirs);
}

#[test]
fn test_class_chars_mixed_classes() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new_unicode_class(ranges: Vec<ClassUnicodeRange>) -> Hir {
            let cls_unicode = ClassUnicode::new(ranges);
            Hir {
                kind: HirKind::Class(Class::Unicode(cls_unicode)),
                props: Properties::default(),
            }
        }

        fn new_bytes_class(ranges: Vec<ClassBytesRange>) -> Hir {
            let cls_bytes = ClassBytes::new(ranges);
            Hir {
                kind: HirKind::Class(Class::Bytes(cls_bytes)),
                props: Properties::default(),
            }
        }
    }

    let hirs: Vec<Hir> = vec![
        TestHir::new_unicode_class(vec![ClassUnicodeRange { start: 'A', end: 'Z' }]),
        TestHir::new_bytes_class(vec![ClassBytesRange { start: 0x41, end: 0x5A }]), // ASCII for 'A' to 'Z'
        TestHir::new_unicode_class(vec![ClassUnicodeRange { start: '!', end: '/' }]),
    ];

    let result = class_chars(&hirs);
}

#[test]
fn test_class_chars_empty_class() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new_empty_unicode_class() -> Hir {
            let cls_unicode = ClassUnicode::empty();
            Hir {
                kind: HirKind::Class(Class::Unicode(cls_unicode)),
                props: Properties::default(),
            }
        }

        fn new_empty_bytes_class() -> Hir {
            let cls_bytes = ClassBytes::empty();
            Hir {
                kind: HirKind::Class(Class::Bytes(cls_bytes)),
                props: Properties::default(),
            }
        }
    }

    let hirs: Vec<Hir> = vec![
        TestHir::new_empty_unicode_class(),
        TestHir::new_empty_bytes_class(),
    ];

    let result = class_chars(&hirs);
}

#[test]
fn test_class_chars_full_range_class() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new_full_unicode_class() -> Hir {
            let cls_unicode = ClassUnicode::new(vec![ClassUnicodeRange { start: '\u{0000}', end: '\u{10FFFF}' }]); // Full Unicode range
            Hir {
                kind: HirKind::Class(Class::Unicode(cls_unicode)),
                props: Properties::default(),
            }
        }

        fn new_full_bytes_class() -> Hir {
            let cls_bytes = ClassBytes::new(vec![ClassBytesRange { start: 0x00, end: 0xFF }]); // All byte values
            Hir {
                kind: HirKind::Class(Class::Bytes(cls_bytes)),
                props: Properties::default(),
            }
        }
    }

    let hirs: Vec<Hir> = vec![
        TestHir::new_full_unicode_class(),
        TestHir::new_full_bytes_class(),
    ];

    let result = class_chars(&hirs);
}

