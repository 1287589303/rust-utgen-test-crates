// Answer 0

#[test]
fn test_concat_simple_literals() {
    let hir = Hir::concat(vec![
        Hir::concat(vec![
            Hir::literal([b'a']),
            Hir::literal([b'b']),
            Hir::literal([b'c']),
        ]),
    ]);
    let _result = hir;
}

#[test]
fn test_concat_with_nested_concats() {
    let hir = Hir::concat(vec![
        Hir::concat(vec![
            Hir::literal([b'a']),
            Hir::literal([b'b']),
        ]),
        Hir::concat(vec![
            Hir::literal([b'c']),
            Hir::literal([b'd']),
        ]),
    ]);
    let _result = hir;
}

#[test]
fn test_concat_adjacent_literals() {
    let hir = Hir::concat(vec![
        Hir::concat(vec![
            Hir::literal([b'a']),
            Hir::literal([b'b']),
            Hir::literal([b'c']),
        ]),
        Hir::concat(vec![
            Hir::literal([b'd']),
            Hir::literal([b'e']),
        ]),
    ]);
    let _result = hir;
}

#[test]
fn test_concat_empty_before_literals() {
    let hir = Hir::concat(vec![
        Hir::empty(),
        Hir::literal([b'a']),
        Hir::literal([b'b']),
    ]);
    let _result = hir;
}

#[test]
fn test_concat_with_only_literals() {
    let hir = Hir::concat(vec![
        Hir::literal([b'a']),
        Hir::literal([b'b']),
        Hir::literal([b'c']),
    ]);
    let _result = hir;
}

#[test]
fn test_concat_with_multiple_layers() {
    let hir = Hir::concat(vec![
        Hir::concat(vec![
            Hir::literal([b'a']),
            Hir::concat(vec![Hir::literal([b'b']), Hir::literal([b'c'])]),
        ]),
        Hir::literal([b'd']),
    ]);
    let _result = hir;
}

