// Answer 0

#[test]
fn test_deserialize_any_number_integer() {
    let value = Value::Number(Number { n: N::from_i64(42).unwrap() });
    let visitor = MyVisitor;
    value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_number_float() {
    let value = Value::Number(Number { n: N::from_f64(3.14).unwrap() });
    let visitor = MyVisitor;
    value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_number_zero() {
    let value = Value::Number(Number { n: N::from_i64(0).unwrap() });
    let visitor = MyVisitor;
    value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_number_negative() {
    let value = Value::Number(Number { n: N::from_f64(-42.0).unwrap() });
    let visitor = MyVisitor;
    value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_number_infinity() {
    let value = Value::Number(Number { n: N::from_f64(f64::INFINITY).unwrap() });
    let visitor = MyVisitor;
    value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_number_negative_infinity() {
    let value = Value::Number(Number { n: N::from_f64(f64::NEG_INFINITY).unwrap() });
    let visitor = MyVisitor;
    value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_number_nan() {
    let value = Value::Number(Number { n: N::from_f64(f64::NAN).unwrap() });
    let visitor = MyVisitor;
    value.deserialize_any(visitor);
}

struct MyVisitor;

impl<'de> Visitor<'de> for MyVisitor {
    type Value = ();
    
    fn visit_unit(self) -> Result<Self::Value, Error> {
        Ok(())
    }
    
    fn visit_bool(self, _: bool) -> Result<Self::Value, Error> {
        Ok(())
    }
    
    fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, Error> {
        Ok(())
    }

    fn visit_seq<V>(self, _: V) -> Result<Self::Value, Error>
    where
        V: SeqAccess<'de>,
    {
        Ok(())
    }
    
    fn visit_map<V>(self, _: V) -> Result<Self::Value, Error>
    where
        V: MapAccess<'de>,
    {
        Ok(())
    }
}

