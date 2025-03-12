// Answer 0

#[test]
fn test_repetition_min_zero_max_zero() {
    let sub_hir = Hir::empty(); // sub is an empty Hir
    let rep = Repetition {
        min: 0,
        max: Some(0),
        greedy: true,
        sub: Box::new(sub_hir),
    };
    let _result = Hir::repetition(rep);
}

#[test]
fn test_repetition_min_one_max_one() {
    let sub_hir = Hir::char('a'); // sub is a single character Hir
    let rep = Repetition {
        min: 1,
        max: Some(1),
        greedy: true,
        sub: Box::new(sub_hir),
    };
    let _result = Hir::repetition(rep);
}

#[test]
fn test_repetition_min_zero_non_empty() {
    let sub_hir = Hir::empty(); // sub is an empty Hir
    let rep = Repetition {
        min: 0,
        max: Some(0),
        greedy: true,
        sub: Box::new(sub_hir),
    };
    // simulate the condition where static_explicit_captures_len is > 0
    let _result = Hir::repetition(rep);
}

