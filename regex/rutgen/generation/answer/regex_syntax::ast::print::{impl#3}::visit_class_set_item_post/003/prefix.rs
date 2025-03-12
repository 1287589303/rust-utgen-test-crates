// Answer 0

#[test]
fn test_visit_class_set_item_post_perl_digit_negated() {
    let kind = ast::ClassPerlKind::Digit;
    let ast = ast::ClassSetItem::Perl(ast::ClassPerl {
        span: Span::default(),
        kind,
        negated: true,
    });
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.visit_class_set_item_post(&ast);
}

#[test]
fn test_visit_class_set_item_post_perl_digit_non_negated() {
    let kind = ast::ClassPerlKind::Digit;
    let ast = ast::ClassSetItem::Perl(ast::ClassPerl {
        span: Span::default(),
        kind,
        negated: false,
    });
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.visit_class_set_item_post(&ast);
}

#[test]
fn test_visit_class_set_item_post_perl_space_negated() {
    let kind = ast::ClassPerlKind::Space;
    let ast = ast::ClassSetItem::Perl(ast::ClassPerl {
        span: Span::default(),
        kind,
        negated: true,
    });
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.visit_class_set_item_post(&ast);
}

#[test]
fn test_visit_class_set_item_post_perl_space_non_negated() {
    let kind = ast::ClassPerlKind::Space;
    let ast = ast::ClassSetItem::Perl(ast::ClassPerl {
        span: Span::default(),
        kind,
        negated: false,
    });
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.visit_class_set_item_post(&ast);
}

#[test]
fn test_visit_class_set_item_post_perl_word_negated() {
    let kind = ast::ClassPerlKind::Word;
    let ast = ast::ClassSetItem::Perl(ast::ClassPerl {
        span: Span::default(),
        kind,
        negated: true,
    });
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.visit_class_set_item_post(&ast);
}

#[test]
fn test_visit_class_set_item_post_perl_word_non_negated() {
    let kind = ast::ClassPerlKind::Word;
    let ast = ast::ClassSetItem::Perl(ast::ClassPerl {
        span: Span::default(),
        kind,
        negated: false,
    });
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.visit_class_set_item_post(&ast);
}

