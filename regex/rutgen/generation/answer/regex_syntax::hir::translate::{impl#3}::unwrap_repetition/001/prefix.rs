// Answer 0

#[test]
fn test_unwrap_repetition_expr() {
    let frame = HirFrame::Expr(Hir { kind: HirKind::from(0), props: Properties::default() });
    frame.unwrap_repetition();
}

#[test]
fn test_unwrap_repetition_literal() {
    let frame = HirFrame::Literal(vec![b'a', b'b', b'c']);
    frame.unwrap_repetition();
}

#[test]
fn test_unwrap_repetition_class_unicode() {
    let frame = HirFrame::ClassUnicode(ClassUnicode { span: Span::default(), negated: false, kind: ClassUnicodeKind::from(0) });
    frame.unwrap_repetition();
}

#[test]
fn test_unwrap_repetition_class_bytes() {
    let frame = HirFrame::ClassBytes(ClassBytes { set: IntervalSet::default() });
    frame.unwrap_repetition();
}

#[test]
fn test_unwrap_repetition_group() {
    let frame = HirFrame::Group { old_flags: Flags::default() };
    frame.unwrap_repetition();
}

#[test]
fn test_unwrap_repetition_concat() {
    let frame = HirFrame::Concat;
    frame.unwrap_repetition();
}

#[test]
fn test_unwrap_repetition_alternation() {
    let frame = HirFrame::Alternation;
    frame.unwrap_repetition();
}

#[test]
fn test_unwrap_repetition_alternation_branch() {
    let frame = HirFrame::AlternationBranch;
    frame.unwrap_repetition();
}

