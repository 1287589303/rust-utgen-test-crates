// Answer 0

#[test]
fn test_visit_i64_min() {
    let visitor = IgnoredAny;
    let result: Result<IgnoredAny, _> = visitor.visit_i64(-9223372036854775808);
}

#[test]
fn test_visit_i64_neg() {
    let visitor = IgnoredAny;
    let result: Result<IgnoredAny, _> = visitor.visit_i64(-1);
}

#[test]
fn test_visit_i64_zero() {
    let visitor = IgnoredAny;
    let result: Result<IgnoredAny, _> = visitor.visit_i64(0);
}

#[test]
fn test_visit_i64_pos() {
    let visitor = IgnoredAny;
    let result: Result<IgnoredAny, _> = visitor.visit_i64(1);
}

#[test]
fn test_visit_i64_max() {
    let visitor = IgnoredAny;
    let result: Result<IgnoredAny, _> = visitor.visit_i64(9223372036854775807);
}

