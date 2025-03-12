// Answer 0

#[test]
fn test_flatten_alternation_with_literals() {
    let hir = Hir::alternation(vec![
        Hir::literal("a".into()),
        Hir::literal("b".into()),
    ]);
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_alternation_with_classes() {
    let hir = Hir::alternation(vec![
        Hir::class(vec![b'a', b'b']),
        Hir::class(vec![b'1', b'2']),
    ]);
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_alternation_with_lookaheads() {
    let hir = Hir::alternation(vec![
        Hir::look(Hir::literal("x".into())),
        Hir::look(Hir::literal("y".into())),
    ]);
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_alternation_with_repetitions() {
    let hir = Hir::alternation(vec![
        Hir::repetition(hir::Repetition::new(Hir::literal("z".into()), 1..=3)),
        Hir::repetition(hir::Repetition::new(Hir::literal("w".into()), 2..=4)),
    ]);
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_alternation_with_captures() {
    let hir = Hir::alternation(vec![
        Hir::Capture {
            sub: Box::new(Hir::literal("test".into())),
            name: None,
            index: 0,
        },
        Hir::literal("no_capture".into()),
    ]);
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_empty_alternation() {
    let hir = Hir::alternation(vec![]);
    let _result = flatten(&hir);
}

#[test]
fn test_flatten_single_element_alternation() {
    let hir = Hir::alternation(vec![Hir::literal("single".into())]);
    let _result = flatten(&hir);
}

