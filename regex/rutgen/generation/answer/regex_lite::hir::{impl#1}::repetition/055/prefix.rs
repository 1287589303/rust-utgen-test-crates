// Answer 0

#[test]
fn test_repetition_min_zero_max_zero() {
    let sub_hir = Hir::empty();
    let rep = Repetition {
        min: 0,
        max: Some(0),
        greedy: true,
        sub: Box::new(sub_hir),
    };
    let result = Hir::repetition(rep);
}

#[test]
fn test_repetition_min_one_max_one() {
    let sub_hir = Hir::char('a');
    let rep = Repetition {
        min: 1,
        max: Some(1),
        greedy: true,
        sub: Box::new(sub_hir),
    };
    let result = Hir::repetition(rep);
}

#[test]
fn test_repetition_min_one_max_zero() {
    let sub_hir = Hir::class(Class::new()); // Assuming Class has a new method.
    let rep = Repetition {
        min: 1,
        max: Some(0),
        greedy: false,
        sub: Box::new(sub_hir),
    };
    let result = Hir::repetition(rep);
}

#[test]
fn test_repetition_min_zero_max_none() {
    let sub_hir = Hir::look(Look::new()); // Assuming Look has a new method.
    let rep = Repetition {
        min: 0,
        max: None,
        greedy: true,
        sub: Box::new(sub_hir),
    };
    let result = Hir::repetition(rep);
}

