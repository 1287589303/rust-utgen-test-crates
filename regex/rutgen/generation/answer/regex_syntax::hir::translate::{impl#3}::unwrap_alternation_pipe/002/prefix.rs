// Answer 0

#[test]
fn test_unwrap_alternation_pipe_with_alternation_branch() {
    let frame = HirFrame::AlternationBranch;
    frame.unwrap_alternation_pipe();
}

#[test]
#[should_panic]
fn test_unwrap_alternation_pipe_with_invalid_frame() {
    let frame = HirFrame::Expr(Hir { kind: HirKind::Literal, props: Properties::default() });
    frame.unwrap_alternation_pipe();
}

