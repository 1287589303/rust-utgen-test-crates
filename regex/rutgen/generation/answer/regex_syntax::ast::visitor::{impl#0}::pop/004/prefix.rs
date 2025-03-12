// Answer 0

#[test]
fn test_pop_concat_non_empty_tail() {
    struct DummyVisitor;

    let ast1 = ast::Literal(Box::new(Literal::new('a')));
    let ast2 = ast::Literal(Box::new(Literal::new('b')));
    let ast3 = ast::Literal(Box::new(Literal::new('c')));
    
    let head = &ast1;
    let tail = &vec![ast2, ast3];

    let induct = Frame::Concat {
        head: head,
        tail: tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(induct);

    // Call the function, expected is Some(Frame::Concat { ... })
    result;
}

#[test]
fn test_pop_concat_non_empty_tail_multiple() {
    struct DummyVisitor;

    let ast1 = ast::Literal(Box::new(Literal::new('x')));
    let ast2 = ast::Literal(Box::new(Literal::new('y')));
    let ast3 = ast::Literal(Box::new(Literal::new('z')));
    
    let head = &ast1;
    let tail = &vec![ast2, ast3];

    let induct = Frame::Concat {
        head: head,
        tail: tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(induct);

    // Call the function, expected is Some(Frame::Concat { ... })
    result;
}

