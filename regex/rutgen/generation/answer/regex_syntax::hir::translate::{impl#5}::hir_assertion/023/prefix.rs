// Answer 0

#[test]
fn test_hir_assertion_start_line() {
    struct TestVisitor {
        trans: Translator,
    }

    let mut visitor = TestVisitor {
        trans: Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                multi_line: Some(true),
                crlf: Some(false),
                ..Flags::default()
            }),
            utf8: false,
            line_terminator: b'\n',
        },
    };

    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartLine,
    };

    visitor.trans.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_end_line() {
    struct TestVisitor {
        trans: Translator,
    }

    let mut visitor = TestVisitor {
        trans: Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                multi_line: Some(true),
                crlf: Some(false),
                ..Flags::default()
            }),
            utf8: false,
            line_terminator: b'\n',
        },
    };

    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::EndLine,
    };

    visitor.trans.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_start_text() {
    struct TestVisitor {
        trans: Translator,
    }

    let mut visitor = TestVisitor {
        trans: Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                multi_line: Some(true),
                crlf: Some(false),
                ..Flags::default()
            }),
            utf8: false,
            line_terminator: b'\n',
        },
    };

    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartText,
    };

    visitor.trans.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_end_text() {
    struct TestVisitor {
        trans: Translator,
    }

    let mut visitor = TestVisitor {
        trans: Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                multi_line: Some(true),
                crlf: Some(false),
                ..Flags::default()
            }),
            utf8: false,
            line_terminator: b'\n',
        },
    };

    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::EndText,
    };

    visitor.trans.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_word_boundary() {
    struct TestVisitor {
        trans: Translator,
    }

    let mut visitor = TestVisitor {
        trans: Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                multi_line: Some(true),
                crlf: Some(false),
                unicode: Some(true),
                ..Flags::default()
            }),
            utf8: false,
            line_terminator: b'\n',
        },
    };

    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundary,
    };

    visitor.trans.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_not_word_boundary() {
    struct TestVisitor {
        trans: Translator,
    }

    let mut visitor = TestVisitor {
        trans: Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                multi_line: Some(true),
                crlf: Some(false),
                unicode: Some(true),
                ..Flags::default()
            }),
            utf8: false,
            line_terminator: b'\n',
        },
    };

    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::NotWordBoundary,
    };

    visitor.trans.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_word_boundary_start() {
    struct TestVisitor {
        trans: Translator,
    }

    let mut visitor = TestVisitor {
        trans: Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                multi_line: Some(true),
                crlf: Some(false),
                unicode: Some(true),
                ..Flags::default()
            }),
            utf8: false,
            line_terminator: b'\n',
        },
    };

    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryStart,
    };

    visitor.trans.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_word_boundary_end() {
    struct TestVisitor {
        trans: Translator,
    }

    let mut visitor = TestVisitor {
        trans: Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                multi_line: Some(true),
                crlf: Some(false),
                unicode: Some(true),
                ..Flags::default()
            }),
            utf8: false,
            line_terminator: b'\n',
        },
    };

    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryEnd,
    };

    visitor.trans.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_word_boundary_start_half() {
    struct TestVisitor {
        trans: Translator,
    }

    let mut visitor = TestVisitor {
        trans: Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                multi_line: Some(true),
                crlf: Some(false),
                unicode: Some(true),
                ..Flags::default()
            }),
            utf8: false,
            line_terminator: b'\n',
        },
    };

    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryStartHalf,
    };

    visitor.trans.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_word_boundary_end_half() {
    struct TestVisitor {
        trans: Translator,
    }

    let mut visitor = TestVisitor {
        trans: Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                multi_line: Some(true),
                crlf: Some(false),
                unicode: Some(true),
                ..Flags::default()
            }),
            utf8: false,
            line_terminator: b'\n',
        },
    };

    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryEndHalf,
    };

    visitor.trans.hir_assertion(&assertion);
}

