// Answer 0

#[test]
fn test_flags_from_ast_case_insensitive() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive), ..Default::default() },
            ast::FlagsItem { kind: ast::FlagsItemKind::Negation, ..Default::default() },
        ],
    };
    let result = Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_multi_line() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine), ..Default::default() },
            ast::FlagsItem { kind: ast::FlagsItemKind::Negation, ..Default::default() },
        ],
    };
    let result = Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_dot_matches_new_line() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine), ..Default::default() },
            ast::FlagsItem { kind: ast::FlagsItemKind::Negation, ..Default::default() },
        ],
    };
    let result = Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_swap_greed() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed), ..Default::default() },
            ast::FlagsItem { kind: ast::FlagsItemKind::Negation, ..Default::default() },
        ],
    };
    let result = Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_unicode() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode), ..Default::default() },
            ast::FlagsItem { kind: ast::FlagsItemKind::Negation, ..Default::default() },
        ],
    };
    let result = Flags::from_ast(&ast);
}

#[test]
fn test_flags_from_ast_crlf() {
    let ast = ast::Flags {
        items: vec![
            ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CRLF), ..Default::default() },
            ast::FlagsItem { kind: ast::FlagsItemKind::Negation, ..Default::default() },
        ],
    };
    let result = Flags::from_ast(&ast);
}

