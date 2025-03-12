// Answer 0

#[test]
fn test_visit_i8_min() {
    let visitor = ContentVisitor { value: PhantomData };
    let result = visitor.visit_i8(-128);
}

#[test]
fn test_visit_i8_middle() {
    let visitor = ContentVisitor { value: PhantomData };
    let result = visitor.visit_i8(0);
}

#[test]
fn test_visit_i8_max() {
    let visitor = ContentVisitor { value: PhantomData };
    let result = visitor.visit_i8(127);
}

