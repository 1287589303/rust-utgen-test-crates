// Answer 0

#[test]
fn test_visit_class_set_binary_op_post_intersection() {
    struct TestVisitor {
        translator: Translator,
    }

    let mut visitor = TestVisitor {
        translator: Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                unicode: Some(false),
                case_insensitive: Some(false),
                ..Flags::default()
            }),
            utf8: true,
            line_terminator: b'\n',
        },
    };

    let op = ast::ClassSetBinaryOp {
        span: Span::default(),
        kind: ast::ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(ClassSet::Item(ClassSetItem::default())),
        rhs: Box::new(ClassSet::Item(ClassSetItem::default())),
    };

    let lhs_cls = ClassBytes::new(vec![ClassBytesRange::default()]);
    let rhs_cls = ClassBytes::new(vec![ClassBytesRange::default()]);
    let mut cls = ClassBytes::new(vec![]);

    visitor.translator.stack.borrow_mut().push(HirFrame::ClassBytes(lhs_cls));
    visitor.translator.stack.borrow_mut().push(HirFrame::ClassBytes(rhs_cls));
    visitor.translator.stack.borrow_mut().push(HirFrame::ClassBytes(cls));

    visitor.visit_class_set_binary_op_post(&op).unwrap();
}

