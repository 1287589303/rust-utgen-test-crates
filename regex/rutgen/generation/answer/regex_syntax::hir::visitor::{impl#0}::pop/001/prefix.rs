// Answer 0

#[test]
fn test_pop_alternation_empty_tail() {
    struct TestHir;
    let hir = TestHir;

    let frame = Frame::Alternation {
        head: &hir,
        tail: &[],
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(frame);
}

#[test]
fn test_pop_alternation_empty_tail_multiple_times() {
    struct TestHir;
    let hir = TestHir;

    let frame = Frame::Alternation {
        head: &hir,
        tail: &[],
    };

    let visitor = HeapVisitor::new();
    let result1 = visitor.pop(frame);
    let result2 = visitor.pop(Frame::Alternation { head: &hir, tail: &[] });
}

