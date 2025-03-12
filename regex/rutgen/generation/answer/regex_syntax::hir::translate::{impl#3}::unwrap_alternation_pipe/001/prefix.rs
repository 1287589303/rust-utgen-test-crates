// Answer 0

#[test]
#[should_panic]
fn test_unwrap_alternation_pipe_not_branch() {
    let frame = HirFrame::Expr(Hir { kind: HirKind::default(), props: Properties::default() });
    frame.unwrap_alternation_pipe();
}

#[test]
#[should_panic]
fn test_unwrap_alternation_pipe_literal() {
    let frame = HirFrame::Literal(vec![b'a']);
    frame.unwrap_alternation_pipe();
}

#[test]
#[should_panic]
fn test_unwrap_alternation_pipe_repetition() {
    let frame = HirFrame::Repetition;
    frame.unwrap_alternation_pipe();
}

#[test]
#[should_panic]
fn test_unwrap_alternation_pipe_group() {
    let flags = Flags::default();
    let frame = HirFrame::Group { old_flags: flags };
    frame.unwrap_alternation_pipe();
}

#[test]
#[should_panic]
fn test_unwrap_alternation_pipe_concat() {
    let frame = HirFrame::Concat;
    frame.unwrap_alternation_pipe();
}

#[test]
#[should_panic]
fn test_unwrap_alternation_pipe_alternation() {
    let frame = HirFrame::Alternation;
    frame.unwrap_alternation_pipe();
}

#[test]
#[should_panic]
fn test_unwrap_alternation_pipe_class_unicode() {
    let class_unicode = hir::ClassUnicode { span: Span::default(), negated: false, kind: ClassUnicodeKind::default() };
    let frame = HirFrame::ClassUnicode(class_unicode);
    frame.unwrap_alternation_pipe();
}

#[test]
#[should_panic]
fn test_unwrap_alternation_pipe_class_bytes() {
    let class_bytes = hir::ClassBytes { set: IntervalSet::default() };
    let frame = HirFrame::ClassBytes(class_bytes);
    frame.unwrap_alternation_pipe();
}

