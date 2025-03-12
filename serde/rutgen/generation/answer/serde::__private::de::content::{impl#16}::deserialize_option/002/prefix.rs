// Answer 0

#[test]
fn test_deserialize_option_with_unit() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();
        fn visit_none(self) -> Result<Self::Value, <ContentDeserializer<'de, ()> as Deserializer<'de>>::Error> {
            Ok(())
        }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, <ContentDeserializer<'de, ()> as Deserializer<'de>>::Error>
        where
            V: Deserializer<'de> {
            Ok(())
        }
        fn visit_unit(self) -> Result<Self::Value, <ContentDeserializer<'de, ()> as Deserializer<'de>>::Error> {
            Ok(())
        }
    }

    let content = Content::Unit;
    let deserializer = ContentDeserializer::new(content);
    let visitor = VisitorImpl;

    let _ = deserializer.deserialize_option(visitor);
}

#[test]
fn test_deserialize_option_with_none() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();
        fn visit_none(self) -> Result<Self::Value, <ContentDeserializer<'de, ()> as Deserializer<'de>>::Error> {
            Ok(())
        }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, <ContentDeserializer<'de, ()> as Deserializer<'de>>::Error>
        where
            V: Deserializer<'de> {
            Ok(())
        }
        fn visit_unit(self) -> Result<Self::Value, <ContentDeserializer<'de, ()> as Deserializer<'de>>::Error> {
            Ok(())
        }
    }

    let content = Content::None;
    let deserializer = ContentDeserializer::new(content);
    let visitor = VisitorImpl;

    let _ = deserializer.deserialize_option(visitor);
}

#[test]
fn test_deserialize_option_with_some() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();
        fn visit_none(self) -> Result<Self::Value, <ContentDeserializer<'de, ()> as Deserializer<'de>>::Error> {
            Ok(())
        }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, <ContentDeserializer<'de, ()> as Deserializer<'de>>::Error>
        where
            V: Deserializer<'de> {
            Ok(())
        }
        fn visit_unit(self) -> Result<Self::Value, <ContentDeserializer<'de, ()> as Deserializer<'de>>::Error> {
            Ok(())
        }
    }

    let content = Content::Some(Box::new(Content::Unit));
    let deserializer = ContentDeserializer::new(content);
    let visitor = VisitorImpl;

    let _ = deserializer.deserialize_option(visitor);
}

