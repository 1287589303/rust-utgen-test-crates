// Answer 0

#[test]
fn test_visit_u64_zero() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContentField;
        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
        fn visit_u64<E>(self, field_index: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match field_index {
                0 => Ok(TagOrContentField::Tag),
                1 => Ok(TagOrContentField::Content),
                _ => Err(de::Error::invalid_value(Unexpected::Unsigned(field_index), &self)),
            }
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_u64(0);
}

#[test]
fn test_visit_u64_one() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContentField;
        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
        fn visit_u64<E>(self, field_index: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match field_index {
                0 => Ok(TagOrContentField::Tag),
                1 => Ok(TagOrContentField::Content),
                _ => Err(de::Error::invalid_value(Unexpected::Unsigned(field_index), &self)),
            }
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_u64(1);
}

#[test]
#[should_panic]
fn test_visit_u64_greater_than_one() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContentField;
        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
        fn visit_u64<E>(self, field_index: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match field_index {
                0 => Ok(TagOrContentField::Tag),
                1 => Ok(TagOrContentField::Content),
                _ => Err(de::Error::invalid_value(Unexpected::Unsigned(field_index), &self)),
            }
        }
    }

    let visitor = TestVisitor;
    let _result = visitor.visit_u64(2);
}

