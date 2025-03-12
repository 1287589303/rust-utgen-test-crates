// Answer 0

#[test]
fn test_from_ast_case_insensitive() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
        ],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_multi_line() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
            },
        ],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_dot_matches_new_line() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
            },
        ],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_swap_greed() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
            },
        ],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_unicode() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
            },
        ],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_crlf() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::CRLF),
            },
        ],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_mixed_flags() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Negation,
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
        ],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_from_ast_all_flags_with_negation() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Negation,
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(ast::Flag::CRLF),
            },
        ],
    };
    Flags::from_ast(&ast);
}

