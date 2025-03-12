// Answer 0

#[test]
fn test_concat_with_literals_and_non_empty() {
    let hir = Hir::concat(vec![
        Hir::literal([b'a']),
        Hir::literal([b'b']),
        Hir::empty(),
        Hir::literal([b'c']),
    ]);
    let expected = Hir::literal("abc".as_bytes());
    let _result = hir;
}

#[test]
fn test_concat_with_multiple_literals_and_empty() {
    let hir = Hir::concat(vec![
        Hir::literal([b'd']),
        Hir::empty(),
        Hir::literal([b'e']),
        Hir::literal([b'f']),
    ]);
    let expected = Hir::literal("def".as_bytes());
    let _result = hir;
}

#[test]
fn test_concat_with_empty_interleaved() {
    let hir = Hir::concat(vec![
        Hir::empty(),
        Hir::literal([b'g']),
        Hir::literal([b'h']),
        Hir::empty(),
    ]);
    let expected = Hir::literal("gh".as_bytes());
    let _result = hir;
}

#[test]
fn test_concat_ignoring_empty_with_literals() {
    let hir = Hir::concat(vec![
        Hir::literal([b'i']),
        Hir::literal([b'j']),
        Hir::empty(),
        Hir::literal([b'k']),
    ]);
    let expected = Hir::literal("ijk".as_bytes());
    let _result = hir;
}

