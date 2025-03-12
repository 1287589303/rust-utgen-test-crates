// Answer 0

#[test]
fn test_alternation_with_literal_and_sub_alternation() {
    let hir = Hir::alternation(vec![
        Hir::literal(b'a'),
        Hir::alternation(vec![
            Hir::literal(b'b'),
            Hir::literal(b'c'),
        ]),
    ]);
    let _ = hir; // Just to call the function
}

#[test]
fn test_alternation_with_singleton_char_and_sub_alternation() {
    let hir = Hir::alternation(vec![
        Hir::alternation(vec![
            Hir::literal("x".as_bytes()),
            Hir::literal("y".as_bytes()),
        ]),
        Hir::literal("z".as_bytes()),
    ]);
    let _ = hir; // Just to call the function
}

#[test]
fn test_alternation_with_non_empty_classes() {
    let hir = Hir::alternation(vec![
        Hir::literal(b'd'),
        Hir::class(Class::Unicode(ClassUnicode::new(vec![
            ClassUnicodeRange { start: 'a', end: 'z' },
        ]))),
    ]);
    let _ = hir; // Just to call the function
}

#[test]
fn test_alternation_with_multiple_literals_and_sub() {
    let hir = Hir::alternation(vec![
        Hir::literal(b'x'),
        Hir::literal(b'y'),
        Hir::alternation(vec![
            Hir::literal(b'z'),
            Hir::literal(b'w'),
        ]),
    ]);
    let _ = hir; // Just to call the function
}

#[test]
fn test_alternation_with_combined_classes() {
    let hir = Hir::alternation(vec![
        Hir::class(Class::Unicode(ClassUnicode::new(vec![
            ClassUnicodeRange { start: 'A', end: 'C' },
        ]))),
        Hir::class(Class::Unicode(ClassUnicode::new(vec![
            ClassUnicodeRange { start: 'D', end: 'F' },
        ]))),
    ]);
    let _ = hir; // Just to call the function
}

