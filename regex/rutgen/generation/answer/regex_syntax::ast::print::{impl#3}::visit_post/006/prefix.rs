// Answer 0

#[test]
fn test_visit_post_class_perl_digit_negated() {
    let span = Span { start: 0, end: 3 }; // Example span
    let kind = ClassPerlKind::Digit; // Digit kind
    let ast = Ast::ClassPerl(Box::new(ClassPerl { span, kind, negated: true }));
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_perl_digit_non_negated() {
    let span = Span { start: 0, end: 3 }; // Example span
    let kind = ClassPerlKind::Digit; // Digit kind
    let ast = Ast::ClassPerl(Box::new(ClassPerl { span, kind, negated: false }));
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_perl_space_negated() {
    let span = Span { start: 0, end: 3 }; // Example span
    let kind = ClassPerlKind::Space; // Space kind
    let ast = Ast::ClassPerl(Box::new(ClassPerl { span, kind, negated: true }));
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_perl_space_non_negated() {
    let span = Span { start: 0, end: 3 }; // Example span
    let kind = ClassPerlKind::Space; // Space kind
    let ast = Ast::ClassPerl(Box::new(ClassPerl { span, kind, negated: false }));
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_perl_word_negated() {
    let span = Span { start: 0, end: 3 }; // Example span
    let kind = ClassPerlKind::Word; // Word kind
    let ast = Ast::ClassPerl(Box::new(ClassPerl { span, kind, negated: true }));
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_perl_word_non_negated() {
    let span = Span { start: 0, end: 3 }; // Example span
    let kind = ClassPerlKind::Word; // Word kind
    let ast = Ast::ClassPerl(Box::new(ClassPerl { span, kind, negated: false }));
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.visit_post(&ast);
}

