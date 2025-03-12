// Answer 0

#[test]
fn test_fmt_group_pre_non_capturing_with_negation_flags() {
    let mut output = String::new();
    let flags = Flags {
        span: Span::default(),
        items: vec![
            FlagsItem {
                kind: FlagsItemKind::Negation,
                span: Span::default(),
            },
        ],
    };
    let group = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::NonCapturing(Box::new(flags)),
        ast: Box::new(ast::Ast::default()),
    };
    let mut writer = Writer { wtr: &mut output };
    let _result = writer.fmt_group_pre(&group);
}

#[test]
#[should_panic]
fn test_fmt_group_pre_non_capturing_flags_fails() {
    let mut output = String::new();
    let flags = Flags {
        span: Span::default(),
        items: vec![
            FlagsItem {
                kind: FlagsItemKind::Negation,
                span: Span::default(),
            },
        ],
    };
    let group = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::NonCapturing(Box::new(flags)),
        ast: Box::new(ast::Ast::default()),
    };
    let mut writer = Writer { wtr: &mut output };
    let _result = writer.fmt_group_pre(&group);
}

