// Answer 0

#[test]
fn test_fmt_class_perl_word_non_negated() {
    let mut output = String::new();
    let ast = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };
    let mut writer = Writer { wtr: &mut output };
    let _ = writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_space_non_negated() {
    let mut output = String::new();
    let ast = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };
    let mut writer = Writer { wtr: &mut output };
    let _ = writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_digit_non_negated() {
    let mut output = String::new();
    let ast = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };
    let mut writer = Writer { wtr: &mut output };
    let _ = writer.fmt_class_perl(&ast);
}

