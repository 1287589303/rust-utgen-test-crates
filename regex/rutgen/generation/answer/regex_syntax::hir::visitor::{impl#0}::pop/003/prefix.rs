// Answer 0

#[test]
fn test_pop_concat_empty_tail() {
    struct TestHir;
    let test_hir = TestHir;

    let tail: &'static [TestHir] = &[];
    let induct = Frame::Concat {
        head: &test_hir,
        tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(induct);
}

#[test]
fn test_pop_alternation_empty_tail() {
    struct TestHir;
    let test_hir = TestHir;

    let tail: &'static [TestHir] = &[];
    let induct = Frame::Alternation {
        head: &test_hir,
        tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(induct);
}

