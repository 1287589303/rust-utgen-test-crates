// Answer 0

#[test]
fn test_deserialize_option_valid() {
    struct ValidVisitor;

    impl<'de> de::Visitor<'de> for ValidVisitor {
        type Value = Option<String>;

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value>
        where
            D: de::Deserializer<'de>,
        {
            Ok(Some(String::from("valid")))
        }

        fn visit_none(self) -> Result<Self::Value> {
            Ok(None)
        }
    }

    let mut deserializer = Deserializer {
        read: SliceRead::new(&[]),
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let key = MapKey { de: &mut deserializer };
    let _result: Option<String> = key.deserialize_option(ValidVisitor).unwrap();
}

#[test]
fn test_deserialize_option_with_alternative_valid() {
    struct AlternativeValidVisitor;

    impl<'de> de::Visitor<'de> for AlternativeValidVisitor {
        type Value = Option<u32>;

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value>
        where
            D: de::Deserializer<'de>,
        {
            Ok(Some(42))
        }

        fn visit_none(self) -> Result<Self::Value> {
            Ok(None)
        }
    }

    let mut deserializer = Deserializer {
        read: SliceRead::new(&[]),
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let key = MapKey { de: &mut deserializer };
    let _result: Option<u32> = key.deserialize_option(AlternativeValidVisitor).unwrap();
}

#[test]
fn test_deserialize_option_with_custom_struct() {
    struct CustomVisitor;

    #[derive(Debug)]
    struct CustomStruct {
        value: String,
    }

    impl<'de> de::Visitor<'de> for CustomVisitor {
        type Value = Option<CustomStruct>;

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value>
        where
            D: de::Deserializer<'de>,
        {
            Ok(Some(CustomStruct { value: String::from("custom") }))
        }

        fn visit_none(self) -> Result<Self::Value> {
            Ok(None)
        }
    }

    let mut deserializer = Deserializer {
        read: SliceRead::new(&[]),
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let key = MapKey { de: &mut deserializer };
    let _result: Option<CustomStruct> = key.deserialize_option(CustomVisitor).unwrap();
}

