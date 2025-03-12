// Answer 0

#[test]
fn test_deserialize_empty_tuple_struct() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn visit_unit(self) -> Result<Self::Value, de::Error> {
            Ok(())
        }
    }

    let content = Content::Seq(vec![]);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    let _ = deserializer.deserialize_tuple_struct("Test", 0, TestVisitor);
}

#[test]
fn test_deserialize_non_empty_tuple_struct() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;
        fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, de::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut vec = vec![];
            while let Some(value) = visitor.next_element()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    let content = Content::Seq(vec![Content::I32(1), Content::I32(2), Content::I32(3)]);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    let _ = deserializer.deserialize_tuple_struct("Test", 3, TestVisitor);
}

#[test]
#[should_panic]
fn test_deserialize_invalid_content() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn visit_unit(self) -> Result<Self::Value, de::Error> {
            Ok(())
        }
    }

    let content = Content::Bool(true);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    let _ = deserializer.deserialize_tuple_struct("Test", 1, TestVisitor);
}

