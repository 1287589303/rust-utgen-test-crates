// Answer 0

#[test]
fn test_child_binary_lhs_item_literal() {
    let lhs_item = ast::ClassSet::Item(ast::ClassSetItem::Literal(Literal { /* initialize with valid data */ }));
    let op = ClassSetBinaryOp { span: Span { /* valid span */ }, kind: ClassSetBinaryOpKind::SomeKind, lhs: Box::new(lhs_item.clone()), rhs: Box::new(lhs_item.clone()) };
    let frame = ClassFrame::BinaryLHS { op: &op, lhs: &lhs_item, rhs: &lhs_item };
    let _result = frame.child();
}

#[test]
fn test_child_binary_lhs_item_range() {
    let lhs_item = ast::ClassSet::Item(ast::ClassSetItem::Range(ClassSetRange { /* initialize with valid data */ }));
    let op = ClassSetBinaryOp { span: Span { /* valid span */ }, kind: ClassSetBinaryOpKind::SomeKind, lhs: Box::new(lhs_item.clone()), rhs: Box::new(lhs_item.clone()) };
    let frame = ClassFrame::BinaryLHS { op: &op, lhs: &lhs_item, rhs: &lhs_item };
    let _result = frame.child();
}

#[test]
fn test_child_binary_lhs_item_ascii() {
    let lhs_item = ast::ClassSet::Item(ast::ClassSetItem::Ascii(ClassAscii { /* initialize with valid data */ }));
    let op = ClassSetBinaryOp { span: Span { /* valid span */ }, kind: ClassSetBinaryOpKind::SomeKind, lhs: Box::new(lhs_item.clone()), rhs: Box::new(lhs_item.clone()) };
    let frame = ClassFrame::BinaryLHS { op: &op, lhs: &lhs_item, rhs: &lhs_item };
    let _result = frame.child();
}

#[test]
fn test_child_binary_lhs_item_unicode() {
    let lhs_item = ast::ClassSet::Item(ast::ClassSetItem::Unicode(ClassUnicode { /* initialize with valid data */ }));
    let op = ClassSetBinaryOp { span: Span { /* valid span */ }, kind: ClassSetBinaryOpKind::SomeKind, lhs: Box::new(lhs_item.clone()), rhs: Box::new(lhs_item.clone()) };
    let frame = ClassFrame::BinaryLHS { op: &op, lhs: &lhs_item, rhs: &lhs_item };
    let _result = frame.child();
}

#[test]
fn test_child_binary_lhs_item_perl() {
    let lhs_item = ast::ClassSet::Item(ast::ClassSetItem::Perl(ClassPerl { /* initialize with valid data */ }));
    let op = ClassSetBinaryOp { span: Span { /* valid span */ }, kind: ClassSetBinaryOpKind::SomeKind, lhs: Box::new(lhs_item.clone()), rhs: Box::new(lhs_item.clone()) };
    let frame = ClassFrame::BinaryLHS { op: &op, lhs: &lhs_item, rhs: &lhs_item };
    let _result = frame.child();
}

#[test]
fn test_child_binary_lhs_item_bracketed() {
    let lhs_item = ast::ClassSet::Item(ast::ClassSetItem::Bracketed(Box::new(ClassBracketed { /* initialize with valid data */ })));
    let op = ClassSetBinaryOp { span: Span { /* valid span */ }, kind: ClassSetBinaryOpKind::SomeKind, lhs: Box::new(lhs_item.clone()), rhs: Box::new(lhs_item.clone()) };
    let frame = ClassFrame::BinaryLHS { op: &op, lhs: &lhs_item, rhs: &lhs_item };
    let _result = frame.child();
}

