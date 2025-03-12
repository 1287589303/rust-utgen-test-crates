// Answer 0

#[test]
fn test_visit_i64_negative_bound() {
    let visitor = MockVisitor {};
    let parser_number = ParserNumber::I64(i64::MIN);
    parser_number.visit(visitor);
}

#[test]
fn test_visit_i64_zero() {
    let visitor = MockVisitor {};
    let parser_number = ParserNumber::I64(0);
    parser_number.visit(visitor);
}

#[test]
fn test_visit_i64_positive_bound() {
    let visitor = MockVisitor {};
    let parser_number = ParserNumber::I64(i64::MAX);
    parser_number.visit(visitor);
}

#[test]
fn test_visit_i64_mid_range() {
    let visitor = MockVisitor {};
    let parser_number = ParserNumber::I64(123456789);
    parser_number.visit(visitor);
}

struct MockVisitor;

impl de::Visitor<'_> for MockVisitor {
    type Value = ();

    fn visit_f64<E>(self, _: f64) -> result::Result<Self::Value, E> {
        unimplemented!()
    }

    fn visit_u64<E>(self, _: u64) -> result::Result<Self::Value, E> {
        unimplemented!()
    }

    fn visit_i64<E>(self, _: i64) -> result::Result<Self::Value, E> {
        unimplemented!()
    }

    fn visit_map<M>(self, _: M) -> result::Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'_>,
    {
        unimplemented!()
    }
}

