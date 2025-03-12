// Answer 0

#[test]
fn test_from_ast_single_negation() {
    let ast_flags = ast::Flags {
        items: vec![ast::FlagsItem {
            kind: ast::FlagsItemKind::Negation,
            span: Span::default(),
        }],
    };
    let _flags = Flags::from_ast(&ast_flags);
}

#[test]
fn test_from_ast_negation_with_ignored_flags() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace),
                span: Span::default(),
            },
        ],
    };
    let _flags = Flags::from_ast(&ast_flags);
}

#[test]
fn test_from_ast_negation_with_other_flags() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CRLF),
                span: Span::default(),
            },
        ],
    };
    let _flags = Flags::from_ast(&ast_flags);
}

#[test]
fn test_from_ast_multiple_negations() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
        ],
    };
    let _flags = Flags::from_ast(&ast_flags);
}

#[test]
fn test_from_ast_negation_followed_by_ignored_whitespace() {
    let ast_flags = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace),
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
                span: Span::default(),
            },
        ],
    };
    let _flags = Flags::from_ast(&ast_flags);
}

