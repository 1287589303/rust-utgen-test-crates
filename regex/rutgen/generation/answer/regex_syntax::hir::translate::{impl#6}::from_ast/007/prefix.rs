// Answer 0

#[test]
fn test_flags_from_ast_case_insensitive_enabled() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
        ],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_multi_line_enabled() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
        ],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_dot_matches_new_line_enabled() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
        ],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_swap_greed_enabled() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
        ],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_unicode_enabled() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
        ],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_crlf_enabled() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CRLF),
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
        ],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_case_insensitive_negated() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
                span: Span::default(),
            },
        ],
    };
    Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_all_flags_negated() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Negation,
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
                span: Span::default(),
            },
            ast::FlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
                span: Span::default(),
            }
        ],
    };
    Flags::from_ast(&ast);
}

