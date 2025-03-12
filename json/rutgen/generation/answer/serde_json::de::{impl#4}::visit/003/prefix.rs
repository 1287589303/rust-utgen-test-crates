// Answer 0

#[test]
fn test_visit_f64_min() {
    let parser_number = ParserNumber::F64(f64::MIN);
    let mut visitor = MockVisitor::new();
    let _ = parser_number.visit(visitor);
}

#[test]
fn test_visit_f64_max() {
    let parser_number = ParserNumber::F64(f64::MAX);
    let mut visitor = MockVisitor::new();
    let _ = parser_number.visit(visitor);
}

#[test]
fn test_visit_f64_nan() {
    let parser_number = ParserNumber::F64(f64::NAN);
    let mut visitor = MockVisitor::new();
    let _ = parser_number.visit(visitor);
}

#[test]
fn test_visit_f64_infinity() {
    let parser_number = ParserNumber::F64(f64::INFINITY);
    let mut visitor = MockVisitor::new();
    let _ = parser_number.visit(visitor);
}

#[test]
fn test_visit_f64_negative_infinity() {
    let parser_number = ParserNumber::F64(f64::NEG_INFINITY);
    let mut visitor = MockVisitor::new();
    let _ = parser_number.visit(visitor);
}

struct MockVisitor {
    value: Option<f64>,
}

impl MockVisitor {
    fn new() -> Self {
        MockVisitor { value: None }
    }
}

impl<'de> de::Visitor<'de> for MockVisitor {
    type Value = ();

    fn visit_f64<E>(self, value: f64) -> result::Result<Self::Value, E> {
        self.value = Some(value);
        Ok(())
    }

    forward_to_deserialize_any! {
        i8, i16, i32, i64, i128, u8, u16, u32, u64, u128,
        bool, char, str, string, bytes, byte_buf,
        option, unit, sequence, map, struct, identifier, enum,
    }
}

