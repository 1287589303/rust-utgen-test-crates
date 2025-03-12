// Answer 0

#[test]
fn test_tuple_variant_zero_length() {
    struct TestVisitor;
    
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any value")
        }

        fn visit_unit<E>(self) -> result::Result<Self::Value, E> where E: de::Error {
            Ok(())
        }
    }

    let mut de = Deserializer {
        read: (),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    
    let result = de.tuple_variant(0, TestVisitor);
}

#[test]
fn test_tuple_variant_one_length() {
    struct TestVisitor;
    
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any value")
        }

        fn visit_unit<E>(self) -> result::Result<Self::Value, E> where E: de::Error {
            Ok(())
        }
    }

    let mut de = Deserializer {
        read: (),
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = de.tuple_variant(1, TestVisitor);
}

#[test]
fn test_tuple_variant_ten_length() {
    struct TestVisitor;
    
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any value")
        }

        fn visit_unit<E>(self) -> result::Result<Self::Value, E> where E: de::Error {
            Ok(())
        }
    }

    let mut de = Deserializer {
        read: (),
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = de.tuple_variant(10, TestVisitor);
}

