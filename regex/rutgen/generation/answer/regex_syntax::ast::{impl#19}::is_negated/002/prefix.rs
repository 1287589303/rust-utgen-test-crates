// Answer 0

#[test]
fn test_is_negated_named_value_not_equal_negated_true() {
    let class_unicode = ClassUnicode {
        span: Span { start: Position(0), end: Position(5) },
        negated: true,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::NotEqual,
            name: String::from("scx"),
            value: String::from("Katakana"),
        },
    };
    class_unicode.is_negated();
}

#[test]
fn test_is_negated_named_value_not_equal_negated_false() {
    let class_unicode = ClassUnicode {
        span: Span { start: Position(0), end: Position(5) },
        negated: false,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::NotEqual,
            name: String::from("scx"),
            value: String::from("Katakana"),
        },
    };
    class_unicode.is_negated();
}

#[test]
fn test_is_negated_other_kind_negated_true() {
    let class_unicode = ClassUnicode {
        span: Span { start: Position(0), end: Position(5) },
        negated: true,
        kind: ClassUnicodeKind::OneLetter('N'),
    };
    class_unicode.is_negated();
}

#[test]
fn test_is_negated_other_kind_negated_false() {
    let class_unicode = ClassUnicode {
        span: Span { start: Position(0), end: Position(5) },
        negated: false,
        kind: ClassUnicodeKind::Named(String::from("category")),
    };
    class_unicode.is_negated();
}

