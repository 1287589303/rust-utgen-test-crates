// Answer 0

#[test]
fn test_unwrap_repetition_valid() {
    let frame = HirFrame::Repetition;
    frame.unwrap_repetition();
}

#[test]
#[should_panic]
fn test_unwrap_repetition_invalid() {
    let frame = HirFrame::Expr(Hir { kind: HirKind::Literal, props: Default::default() });
    frame.unwrap_repetition();
}

