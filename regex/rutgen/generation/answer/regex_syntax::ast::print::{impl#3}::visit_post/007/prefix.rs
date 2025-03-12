// Answer 0

#[test]
fn test_visit_post_class_unicode_one_letter() {
    let mut writer = Writer { wtr: String::new() };
    let ast = Ast::ClassUnicode(Box::new(ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ClassUnicodeKind::OneLetter('a'),
    }));
    let _ = writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_unicode_named() {
    let mut writer = Writer { wtr: String::new() };
    let ast = Ast::ClassUnicode(Box::new(ClassUnicode {
        span: Span::default(),
        negated: true,
        kind: ClassUnicodeKind::Named("Greek".to_string()),
    }));
    let _ = writer.visit_post(&ast);
}

