// Answer 0

#[test]
fn test_repetition_min_greater_than_zero_max_zero() {
    let rep = Repetition {
        min: 1,
        max: Some(0),
        greedy: true,
        sub: Box::new(Hir::empty()),
    };
    let _result = Hir::repetition(rep);
}

#[test]
fn test_repetition_min_greater_than_zero_max_zero_greedy_false() {
    let rep = Repetition {
        min: 3,
        max: Some(0),
        greedy: false,
        sub: Box::new(Hir::char('a')),
    };
    let _result = Hir::repetition(rep);
}

#[test]
fn test_repetition_min_greater_than_zero_max_zero_large_min() {
    let rep = Repetition {
        min: 5,
        max: Some(0),
        greedy: true,
        sub: Box::new(Hir::class(Class::new())),
    };
    let _result = Hir::repetition(rep);
}

