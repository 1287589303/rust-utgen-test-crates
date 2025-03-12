// Answer 0

#[test]
fn test_visit_u8_zero() {
    let visitor = ContentVisitor { value: PhantomData };
    let value: u8 = 0;
    let _ = visitor.visit_u8(value);
}

#[test]
fn test_visit_u8_mid_range() {
    let visitor = ContentVisitor { value: PhantomData };
    let value: u8 = 128;
    let _ = visitor.visit_u8(value);
}

#[test]
fn test_visit_u8_max() {
    let visitor = ContentVisitor { value: PhantomData };
    let value: u8 = 255;
    let _ = visitor.visit_u8(value);
}

