// Answer 0

#[test]
fn test_alternation_empty_new() {
    let hir_empty = Hir::empty();
    let result = Hir::alternation(vec![hir_empty.clone()]);
}

#[test]
fn test_alternation_with_single_nested_alternation() {
    let hir_nested = Hir::alternation(vec![
        Hir::literal([b'a']),
        Hir::literal([b'b']),
    ]);
    let result = Hir::alternation(vec![hir_nested]);
}

