// Answer 0

#[test]
fn test_flatten_concat_with_capture() {
    use regex_syntax::hir::{Hir, HirKind, Class, Literal, Repetition, Capture};

    let sub_capture = Hir::literal("inner".to_string());
    let capture_hir = Hir::capture(Capture::new(sub_capture));

    let literal_hir = Hir::literal("outer".to_string());
    let class_hir = Hir::class(Class::empty());
    
    let repetition_hir = Hir::repetition(Repetition::new(Hir::literal("repeat".to_string())));

    let concat_hir = Hir::concat(vec![literal_hir, class_hir, repetition_hir, capture_hir]);
   
    let result = flatten(&concat_hir);
}

#[test]
fn test_flatten_concat_nested_capture() {
    use regex_syntax::hir::{Hir, HirKind, Class, Literal, Repetition, Capture};

    let inner_capture = Hir::literal("inner_capture".to_string());
    let outer_capture = Hir::capture(Capture::new(inner_capture));

    let class_hir = Hir::class(Class::empty());

    let concat_hir = Hir::concat(vec![outer_capture, class_hir]);
    
    let result = flatten(&concat_hir);
}

#[test]
fn test_flatten_concat_multiple_kinds() {
    use regex_syntax::hir::{Hir, HirKind, Class, Literal, Repetition, Capture};

    let capture_hir = Hir::capture(Capture::new(Hir::literal("dropped".to_string())));

    let literal_hir = Hir::literal("text".to_string());
    let class_hir = Hir::class(Class::from_range(b'a', b'z'));
    let repetition_hir = Hir::repetition(Repetition::new(Hir::literal("one_or_more".to_string())));

    let concat_hir = Hir::concat(vec![literal_hir, class_hir, repetition_hir, capture_hir]);
    
    let result = flatten(&concat_hir);
}

