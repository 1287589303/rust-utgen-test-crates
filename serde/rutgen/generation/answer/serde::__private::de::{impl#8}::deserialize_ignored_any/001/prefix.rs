// Answer 0

#[test]
fn test_deserialize_ignored_any_success() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }
    }

    let mut vec = Vec::new();
    let mut deserializer = FlatMapDeserializer(&mut vec);

    let _ = deserializer.deserialize_ignored_any(TestVisitor);
}

#[test]
#[should_panic]
fn test_deserialize_ignored_any_failure() {
    struct FailingVisitor;

    impl<'de> Visitor<'de> for FailingVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "fail")))
        }
    }

    let mut vec = Vec::new();
    let mut deserializer = FlatMapDeserializer(&mut vec);

    let _ = deserializer.deserialize_ignored_any(FailingVisitor);
}

#[test]
fn test_deserialize_ignored_any_empty_visit() {
    struct EmptyVisitor;

    impl<'de> Visitor<'de> for EmptyVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }
    }

    let mut vec = Vec::new();
    let mut deserializer = FlatMapDeserializer(&mut vec);

    let _ = deserializer.deserialize_ignored_any(EmptyVisitor);
}

#[test]
fn test_deserialize_ignored_any_non_empty_visit() {
    struct NonEmptyVisitor;

    impl<'de> Visitor<'de> for NonEmptyVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }
    }

    let mut vec = vec![None];
    let mut deserializer = FlatMapDeserializer(&mut vec);

    let _ = deserializer.deserialize_ignored_any(NonEmptyVisitor);
}

