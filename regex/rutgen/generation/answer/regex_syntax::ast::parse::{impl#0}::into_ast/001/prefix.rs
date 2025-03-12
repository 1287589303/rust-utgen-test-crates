// Answer 0

#[test]
fn test_into_ast_unicode_negated() {
    let span = Span { start: Position(0), end: Position(1) };
    let cls = ClassUnicode { span, negated: true, kind: ClassUnicodeKind };
    let primitive = Primitive::Unicode(cls);
    let _result = primitive.into_ast();
}

#[test]
fn test_into_ast_unicode_non_negated() {
    let span = Span { start: Position(2), end: Position(3) };
    let cls = ClassUnicode { span, negated: false, kind: ClassUnicodeKind };
    let primitive = Primitive::Unicode(cls);
    let _result = primitive.into_ast();
}

