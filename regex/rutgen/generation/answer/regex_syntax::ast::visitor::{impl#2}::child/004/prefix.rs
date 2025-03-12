// Answer 0

#[test]
fn test_child_union() {
    let head = ast::ClassSetItem::Literal(Literal::from('a'));
    let frame = ClassFrame::Union {
        head: &head,
        tail: &[],
    };
    let _result = frame.child();
}

#[test]
fn test_child_union_non_empty_tail() {
    let head = ast::ClassSetItem::Range(ClassSetRange::new('a', 'z'));
    let tail = vec![ast::ClassSetItem::Literal(Literal::from('b'))];
    let frame = ClassFrame::Union {
        head: &head,
        tail: &tail,
    };
    let _result = frame.child();
}

#[test]
fn test_child_union_empty_item() {
    let head = ast::ClassSetItem::Empty(Span::new(0, 0));
    let frame = ClassFrame::Union {
        head: &head,
        tail: &[],
    };
    let _result = frame.child();
}

