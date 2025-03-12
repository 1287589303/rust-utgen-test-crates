// Answer 0

#[test]
fn test_pop_class_union_non_empty_tail() {
    struct TestAst;
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let tail = vec![
        &TestAst, 
        &TestAst,
    ];
    let induct = ClassFrame::Union {
        head: &tail[0],
        tail: &tail[1..],
    };

    let visitor = TestVisitor;
    let mut visitor_instance = HeapVisitor::new();
    let result = visitor_instance.pop_class(induct);
}

#[test]
fn test_pop_class_union_non_empty_tail_more_elements() {
    struct TestAst;
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let tail = vec![
        &TestAst, 
        &TestAst,
        &TestAst,
    ];
    let induct = ClassFrame::Union {
        head: &tail[0],
        tail: &tail[1..],
    };

    let visitor = TestVisitor;
    let mut visitor_instance = HeapVisitor::new();
    let result = visitor_instance.pop_class(induct);
}

