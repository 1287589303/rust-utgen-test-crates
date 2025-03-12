// Answer 0

#[test]
fn test_deserialize_newtype_struct_valid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_newtype_struct<E>(
            self,
            _value: MapKeyDeserializer<'de>,
        ) -> Result<Self::Value, E> {
            Ok("test_string".to_owned())
        }
    }

    let key = Cow::Borrowed("key");
    let deserializer = MapKeyDeserializer { key };
    let name: &'static str = "test_struct";
    let visitor = TestVisitor;

    let _result = deserializer.deserialize_newtype_struct(name, visitor);
}

#[test]
fn test_deserialize_newtype_struct_empty_name() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_newtype_struct<E>(
            self,
            _value: MapKeyDeserializer<'de>,
        ) -> Result<Self::Value, E> {
            Ok("test_string".to_owned())
        }
    }

    let key = Cow::Borrowed("key");
    let deserializer = MapKeyDeserializer { key };
    let name: &'static str = "";
    let visitor = TestVisitor;

    let _result = deserializer.deserialize_newtype_struct(name, visitor);
}

#[test]
fn test_deserialize_newtype_struct_boundary() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_newtype_struct<E>(
            self,
            _value: MapKeyDeserializer<'de>,
        ) -> Result<Self::Value, E> {
            Ok("boundary_test".to_owned())
        }
    }

    let key = Cow::Borrowed("boundary_key");
    let deserializer = MapKeyDeserializer { key };
    let name: &'static str = "boundary_struct";
    let visitor = TestVisitor;

    let _result = deserializer.deserialize_newtype_struct(name, visitor);
}

