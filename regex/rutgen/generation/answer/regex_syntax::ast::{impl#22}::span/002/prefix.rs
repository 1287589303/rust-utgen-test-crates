// Answer 0

#[test]
fn test_span_bracketed_class_set_item() {
    let span = Span {
        start: Position(0),
        end: Position(10),
    };
    let inner_class_set = ClassSetItem::Literal(Literal {
        span: span.clone(),
        kind: LiteralKind::Unicode,
        c: 'a',
    });
    let bracketed = ClassBracketed {
        span: span.clone(),
        negated: false,
        kind: ClassSet::Item(inner_class_set),
    };
    let class_set_item = ClassSetItem::Bracketed(Box::new(bracketed));
    let _ = class_set_item.span();
}

#[test]
fn test_span_bracketed_class_set_item_with_empty() {
    let span = Span {
        start: Position(0),
        end: Position(5),
    };
    let empty_item = ClassSetItem::Empty(span.clone());
    let bracketed = ClassBracketed {
        span: span.clone(),
        negated: false,
        kind: ClassSet::Item(empty_item),
    };
    let class_set_item = ClassSetItem::Bracketed(Box::new(bracketed));
    let _ = class_set_item.span();
}

#[test]
fn test_span_bracketed_class_set_item_with_unicode() {
    let span = Span {
        start: Position(0),
        end: Position(8),
    };
    let unicode_item = ClassSetItem::Unicode(ClassUnicode {
        span: span.clone(),
        negated: true,
        kind: ClassUnicodeKind::Letter,
    });
    let bracketed = ClassBracketed {
        span: span.clone(),
        negated: false,
        kind: ClassSet::Item(unicode_item),
    };
    let class_set_item = ClassSetItem::Bracketed(Box::new(bracketed));
    let _ = class_set_item.span();
}

#[test]
fn test_span_bracketed_class_set_item_with_perl() {
    let span = Span {
        start: Position(0),
        end: Position(12),
    };
    let perl_item = ClassSetItem::Perl(ClassPerl {
        span: span.clone(),
        kind: ClassPerlKind::Digit,
        negated: false,
    });
    let bracketed = ClassBracketed {
        span: span.clone(),
        negated: true,
        kind: ClassSet::Item(perl_item),
    };
    let class_set_item = ClassSetItem::Bracketed(Box::new(bracketed));
    let _ = class_set_item.span();
}

