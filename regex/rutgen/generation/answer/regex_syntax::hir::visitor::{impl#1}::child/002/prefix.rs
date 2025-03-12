// Answer 0

#[test]
fn test_child_with_empty_concat() {
    struct DummyHir;
    let head = &Hir { kind: HirKind::Dummy, props: Properties::default() };
    let frame = Frame::Concat { head, tail: &[] };
    let result = frame.child();
}

#[test]
fn test_child_with_non_empty_concat() {
    struct DummyHir;
    let head = &Hir { kind: HirKind::Dummy, props: Properties::default() };
    let tail = vec![Hir { kind: HirKind::Dummy, props: Properties::default() }];
    let frame = Frame::Concat { head, tail: &tail };
    let result = frame.child();
}

