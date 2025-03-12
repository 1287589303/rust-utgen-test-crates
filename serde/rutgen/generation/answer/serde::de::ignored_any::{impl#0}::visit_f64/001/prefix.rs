// Answer 0

#[test]
fn test_visit_f64_valid_neg_one() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_f64(-1.0);
}

#[test]
fn test_visit_f64_valid_zero() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_f64(0.0);
}

#[test]
fn test_visit_f64_valid_one() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_f64(1.0);
}

#[test]
fn test_visit_f64_valid_min() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_f64(std::f64::MIN);
}

#[test]
fn test_visit_f64_valid_max() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_f64(std::f64::MAX);
}

#[test]
fn test_visit_f64_valid_infinity() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_f64(std::f64::INFINITY);
}

#[test]
fn test_visit_f64_valid_negative_infinity() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_f64(-std::f64::INFINITY);
}

#[test]
fn test_visit_f64_valid_nan() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_f64(std::f64::NAN);
}

