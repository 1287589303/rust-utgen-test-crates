// Answer 0

#[test]
fn test_deserialize_unit_struct_valid_bool() {
    struct BoolVisitor;
    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a unit struct")
        }
        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }

    let value = Value::Bool(true);
    let visitor = BoolVisitor;
    let _ = value.deserialize_unit_struct("MyUnitStruct", visitor);
}

#[test]
fn test_deserialize_unit_struct_valid_null() {
    struct NullVisitor;
    impl<'de> Visitor<'de> for NullVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a unit struct")
        }
        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }

    let value = Value::Null;
    let visitor = NullVisitor;
    let _ = value.deserialize_unit_struct("MyNullUnitStruct", visitor);
}

#[test]
fn test_deserialize_unit_struct_valid_number() {
    struct NumberVisitor;
    impl<'de> Visitor<'de> for NumberVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a unit struct")
        }
        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }

    let value = Value::Number(Number { n: 0 });
    let visitor = NumberVisitor;
    let _ = value.deserialize_unit_struct("MyNumberUnitStruct", visitor);
}

#[test]
fn test_deserialize_unit_struct_valid_string() {
    struct StringVisitor;
    impl<'de> Visitor<'de> for StringVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a unit struct")
        }
        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }

    let value = Value::String(String::from("example"));
    let visitor = StringVisitor;
    let _ = value.deserialize_unit_struct("MyStringUnitStruct", visitor);
}

#[test]
fn test_deserialize_unit_struct_valid_object() {
    struct ObjectVisitor;
    impl<'de> Visitor<'de> for ObjectVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a unit struct")
        }
        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }

    let value = Value::Object(Map { map: MapImpl::new() });
    let visitor = ObjectVisitor;
    let _ = value.deserialize_unit_struct("MyObjectUnitStruct", visitor);
}

