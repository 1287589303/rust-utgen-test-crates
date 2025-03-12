// Answer 0

#[test]
fn test_deserialize_option_none() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<()>;

        fn visit_none(self) -> Result<Self::Value, Self::Error> {
            Ok(None)
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, Self::Error>
        where
            V: Deserializer<'de>,
        {
            Ok(Some(()))
        }
        
        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(Some(()))
        }
    }

    let content_none = Content::None;
    let deserializer = ContentRefDeserializer::new(&content_none);
    let visitor = TestVisitor;

    let _ = deserializer.deserialize_option(visitor);
}

#[test]
fn test_deserialize_option_some() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<u8>;

        fn visit_none(self) -> Result<Self::Value, Self::Error> {
            Ok(None)
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, Self::Error>
        where
            V: Deserializer<'de>,
        {
            Ok(Some(42))
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(Some(()))
        }
    }

    let content_some = Content::Some(Box::new(Content::U8(10)));
    let deserializer = ContentRefDeserializer::new(&content_some);
    let visitor = TestVisitor;

    let _ = deserializer.deserialize_option(visitor);
}

#[test]
fn test_deserialize_option_unit() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<()>;

        fn visit_none(self) -> Result<Self::Value, Self::Error> {
            Ok(None)
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, Self::Error>
        where
            V: Deserializer<'de>,
        {
            Ok(Some(()))
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(Some(()))
        }
    }

    let content_unit = Content::Unit;
    let deserializer = ContentRefDeserializer::new(&content_unit);
    let visitor = TestVisitor;

    let _ = deserializer.deserialize_option(visitor);
}

#[test]
fn test_deserialize_option_invalid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<u8>;

        fn visit_none(self) -> Result<Self::Value, Self::Error> {
            Ok(None)
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, Self::Error>
        where
            V: Deserializer<'de>,
        {
            Err(Error::custom("Invalid Value"))
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(Some(()))
        }
    }

    let content_invalid = Content::String("Invalid".to_string());
    let deserializer = ContentRefDeserializer::new(&content_invalid);
    let visitor = TestVisitor;

    let _ = deserializer.deserialize_option(visitor);
}

