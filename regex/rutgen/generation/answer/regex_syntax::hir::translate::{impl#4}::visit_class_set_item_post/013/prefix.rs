// Answer 0

#[test]
fn test_visit_class_set_item_post_ascii_unicode() {
    struct TestVisitor {
        translator: Translator,
    }

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let mut visitor = TestVisitor { translator };

    let ast = ast::ClassSetItem::Ascii(ast::ClassAscii {
        span: Span {
            start: Position(0),
            end: Position(1),
        },
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    });

    visitor.translator.stack.borrow_mut().push(HirFrame::ClassUnicode(hir::ClassUnicode::new(vec![])));

    let result = visitor.translator.visit_class_set_item_post(&ast);
    result.unwrap();
}

#[test]
fn test_visit_class_set_item_post_ascii_union() {
    struct TestVisitor {
        translator: Translator,
    }

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let mut visitor = TestVisitor { translator };

    let ast = ast::ClassSetItem::Ascii(ast::ClassAscii {
        span: Span {
            start: Position(0),
            end: Position(1),
        },
        kind: ast::ClassAsciiKind::Digit,
        negated: false,
    });

    visitor.translator.stack.borrow_mut().push(HirFrame::ClassUnicode(hir::ClassUnicode::new(vec![])));

    let result = visitor.translator.visit_class_set_item_post(&ast);
    result.unwrap();
}

