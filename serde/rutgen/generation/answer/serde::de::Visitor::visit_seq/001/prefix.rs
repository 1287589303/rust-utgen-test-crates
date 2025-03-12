// Answer 0

#[test]
fn test_visit_seq_empty() {
    struct EmptySeq;

    impl<'de> SeqAccess<'de> for EmptySeq {
        type Error = &'static str;

        fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: serde::de::DeserializeSeed<'de>,
        {
            Ok(None)
        }
    }

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any sequence")
        }
    }

    let visitor = TestVisitor;
    let _ = visitor.visit_seq(EmptySeq);
}

#[test]
fn test_visit_seq_single_element() {
    struct SingleElementSeq;

    impl<'de> SeqAccess<'de> for SingleElementSeq {
        type Error = &'static str;

        fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: serde::de::DeserializeSeed<'de>,
        {
            Ok(Some(1))
        }
    }

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any sequence")
        }
    }

    let visitor = TestVisitor;
    let _ = visitor.visit_seq(SingleElementSeq);
}

#[test]
fn test_visit_seq_multiple_elements() {
    struct MultipleElementsSeq;

    impl<'de> SeqAccess<'de> for MultipleElementsSeq {
        type Error = &'static str;

        fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: serde::de::DeserializeSeed<'de>,
        {
            Ok(Some(1))
        }
    }

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any sequence")
        }
    }

    let visitor = TestVisitor;
    let _ = visitor.visit_seq(MultipleElementsSeq);
}

