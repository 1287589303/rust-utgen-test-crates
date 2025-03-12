// Answer 0

#[test]
fn test_alternation_singletons_char() {
    let hir = Hir::alternation(vec![
        Hir::literal([b'a']),
        Hir::literal([b'b']),
        Hir::literal([b'c']),
    ]);
    // Expected behavior is that this should simplify to a class:
    let _ = hir;
}

#[test]
fn test_alternation_singletons_byte() {
    let hir = Hir::alternation(vec![
        Hir::literal([b'x']),
        Hir::literal([b'y']),
        Hir::literal([b'z']),
    ]);
    // Expected behavior is that this should simplify to a bytes class:
    let _ = hir;
}

#[test]
fn test_alternation_class_chars() {
    let hir = Hir::alternation(vec![
        Hir::class(Class::Unicode(ClassUnicode::new(vec![
            ClassUnicodeRange::new('d', 'f'),
        ]))),
        Hir::class(Class::Unicode(ClassUnicode::new(vec![
            ClassUnicodeRange::new('a', 'c'),
        ]))),
    ]);
    // Expected behavior is that this should simplify into a character class:
    let _ = hir;
}

#[test]
fn test_alternation_class_bytes() {
    let hir = Hir::alternation(vec![
        Hir::class(Class::Bytes(ClassBytes::new(vec![
            ClassBytesRange::new(120, 122),
        ]))),
        Hir::class(Class::Bytes(ClassBytes::new(vec![
            ClassBytesRange::new(97, 99),
        ]))),
    ]);
    // Expected behavior is that this should simplify into a bytes class:
    let _ = hir;
}

#[test]
fn test_alternation_with_common_prefix() {
    let hir = Hir::alternation(vec![
        Hir::concat(vec![
            Hir::literal(b"xyz"),
            Hir::class(Class::Unicode(ClassUnicode::new(vec![
                ClassUnicodeRange::new('G', 'Z'),
            ]))),
        ]),
        Hir::concat(vec![
            Hir::literal(b"xyz"),
            Hir::class(Class::Unicode(ClassUnicode::new(vec![
                ClassUnicodeRange::new('g', 'z'),
            ]))),
        ]),
    ]);
    // Expected behavior is that this should factor out the common "xyz":
    let _ = hir;
}

