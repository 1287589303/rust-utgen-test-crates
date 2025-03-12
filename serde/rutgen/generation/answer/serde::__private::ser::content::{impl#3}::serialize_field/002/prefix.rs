// Answer 0

#[test]
fn test_serialize_field_with_bool() {
    struct MockMap {
        data: Vec<(&'static str, Content)>,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = std::convert::Infallible;

        fn serialize_entry(&mut self, key: &'static str, value: Content) -> Result<(), Self::Error> {
            self.data.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { data: vec![] };
    let mut variant = SerializeStructVariantAsMapValue {
        map,
        name: "test_variant",
        fields: vec![],
    };
    let key = "is_active";
    let value = &true; // bool implements Serialize
    let _ = variant.serialize_field(key, value);
}

#[test]
fn test_serialize_field_with_u8() {
    struct MockMap {
        data: Vec<(&'static str, Content)>,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = std::convert::Infallible;

        fn serialize_entry(&mut self, key: &'static str, value: Content) -> Result<(), Self::Error> {
            self.data.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { data: vec![] };
    let mut variant = SerializeStructVariantAsMapValue {
        map,
        name: "test_variant",
        fields: vec![],
    };
    let key = "value_u8";
    let value = &255u8; // u8 implements Serialize
    let _ = variant.serialize_field(key, value);
}

#[test]
fn test_serialize_field_with_string() {
    struct MockMap {
        data: Vec<(&'static str, Content)>,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = std::convert::Infallible;

        fn serialize_entry(&mut self, key: &'static str, value: Content) -> Result<(), Self::Error> {
            self.data.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { data: vec![] };
    let mut variant = SerializeStructVariantAsMapValue {
        map,
        name: "test_variant",
        fields: vec![],
    };
    let key = "name";
    let value = &"Rust".to_string(); // String implements Serialize
    let _ = variant.serialize_field(key, value);
}

