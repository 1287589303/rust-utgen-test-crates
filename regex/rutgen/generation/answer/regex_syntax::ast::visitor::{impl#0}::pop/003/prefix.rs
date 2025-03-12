// Answer 0

#[test]
fn test_pop_concat_empty_tail() {
    struct TestAst;
    type Hir = TestAst;
    
    let tail: &[Hir] = &[];
    let induct = Frame::Concat { head: &TestAst, tail };
    let visitor = HeapVisitor::new();

    let result = visitor.pop(induct);
}

#[test]
fn test_pop_alternation_empty_tail() {
    struct TestAst;
    
    let tail: &[Ast] = &[];
    let induct = Frame::Alternation { head: &TestAst, tail };
    let visitor = HeapVisitor::new();

    let result = visitor.pop(induct);
}

