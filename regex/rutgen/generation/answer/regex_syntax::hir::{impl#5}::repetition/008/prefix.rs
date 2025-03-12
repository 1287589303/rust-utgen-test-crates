// Answer 0

#[test]
fn test_repetition_with_min_1_and_max_0() {
    let sub_expression = Hir::literal(Box::new(b"a".to_vec()));
    let properties = Properties::literal(&sub_expression.kind.as_literal().unwrap());
    
    let rep = Repetition {
        min: 1,
        max: Some(0),
        greedy: true,
        sub: Box::new(sub_expression),
    };

    let result = Hir::repetition(rep);
}

#[test]
fn test_repetition_with_min_1_and_max_0_non_greedy() {
    let sub_expression = Hir::literal(Box::new(b"b".to_vec()));
    let properties = Properties::literal(&sub_expression.kind.as_literal().unwrap());

    let rep = Repetition {
        min: 1,
        max: Some(0),
        greedy: false,
        sub: Box::new(sub_expression),
    };

    let result = Hir::repetition(rep);
}

#[test]
fn test_repetition_with_min_1_and_max_0_boundary_case() {
    let sub_expression = Hir::class(Class::new(vec!['c' as u32]));
    
    let rep = Repetition {
        min: 1,
        max: Some(0),
        greedy: true,
        sub: Box::new(sub_expression),
    };

    let result = Hir::repetition(rep);
}

