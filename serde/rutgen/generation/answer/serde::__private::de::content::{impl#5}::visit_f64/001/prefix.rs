// Answer 0

#[test]
fn test_visit_f64_valid() {
    let visitor = ContentVisitor { value: PhantomData };
    let result = visitor.visit_f64(0.0);
}

#[test]
fn test_visit_f64_negative() {
    let visitor = ContentVisitor { value: PhantomData };
    let result = visitor.visit_f64(-1.23);
}

#[test]
fn test_visit_f64_positive() {
    let visitor = ContentVisitor { value: PhantomData };
    let result = visitor.visit_f64(3.14);
}

#[test]
fn test_visit_f64_nan() {
    let visitor = ContentVisitor { value: PhantomData };
    let result = visitor.visit_f64(f64::NAN);
}

#[test]
fn test_visit_f64_infinity() {
    let visitor = ContentVisitor { value: PhantomData };
    let result = visitor.visit_f64(f64::INFINITY);
}

#[test]
fn test_visit_f64_neg_infinity() {
    let visitor = ContentVisitor { value: PhantomData };
    let result = visitor.visit_f64(f64::NEG_INFINITY);
}

#[test]
fn test_visit_f64_large() {
    let visitor = ContentVisitor { value: PhantomData };
    let result = visitor.visit_f64(1.7976931348623157e308); // Largest positive f64
}

#[test]
fn test_visit_f64_small() {
    let visitor = ContentVisitor { value: PhantomData };
    let result = visitor.visit_f64(-1.7976931348623157e308); // Largest negative f64
}

