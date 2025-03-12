// Answer 0

#[test]
fn test_into_class_set_item_error_dot() {
    let span = Span { start: 0, end: 1 };
    let primitive = Primitive::Dot(span);
    let parser = ParserI::new((), ".*");
    let _result = primitive.into_class_set_item(&parser);
}

#[test]
fn test_into_class_set_item_error_assertion() {
    let span = Span { start: 0, end: 1 };
    let assertion = Assertion { span, kind: AssertionKind::StartOfLine };
    let primitive = Primitive::Assertion(assertion);
    let parser = ParserI::new((), "^");
    let _result = primitive.into_class_set_item(&parser);
}

#[test]
fn test_into_class_set_item_error_invalid_type() {
    let span = Span { start: 0, end: 1 };
    let primitive = Primitive::Unicode(ClassUnicode { span, negated: false, kind: ClassUnicodeKind::Other });
    let parser = ParserI::new((), "\\w");
    let _result = primitive.into_class_set_item(&parser);
}

