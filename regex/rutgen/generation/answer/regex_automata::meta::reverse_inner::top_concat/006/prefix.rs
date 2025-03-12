// Answer 0

#[test]
fn test_top_concat_capture_with_concat() {
    use regex_syntax::hir::{Hir, HirKind, Capture};
    use regex_syntax::hir::Class;
    use regex_syntax::hir::Literal;

    let inner_capture = Hir::concat(vec![
        Hir::literal("a".into()),
        Hir::class(vec![Class::new_char_range('b', 'b')])
    ]);

    let top_concat = Hir::concat(vec![
        Hir::capture(Capture { sub: Box::new(inner_capture), span: Default::default() }),
        Hir::literal("c".into())
    ]);

    let result = top_concat(&top_concat);
} 

#[test]
fn test_top_concat_nested_capture_with_concat() {
    use regex_syntax::hir::{Hir, HirKind, Capture};
    use regex_syntax::hir::Literal;

    let nested_capture = Hir::concat(vec![
        Hir::literal("x".into()),
        Hir::literal("y".into())
    ]);

    let top_concat = Hir::concat(vec![
        Hir::capture(Capture { sub: Box::new(nested_capture), span: Default::default() }),
        Hir::literal("z".into())
    ]);

    let result = top_concat(&top_concat);
} 

#[test]
fn test_top_concat_multiple_captures_with_concat() {
    use regex_syntax::hir::{Hir, HirKind, Capture};
    use regex_syntax::hir::Literal;

    let first_capture = Hir::literal("1".into());
    let second_capture = Hir::literal("2".into());

    let top_concat = Hir::concat(vec![
        Hir::capture(Capture { sub: Box::new(first_capture), span: Default::default() }),
        Hir::capture(Capture { sub: Box::new(second_capture), span: Default::default() }),
    ]);

    let result = top_concat(&top_concat);
}

