// Answer 0

#[test]
fn test_visit_content_seq_with_err_variant() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, Box<dyn de::Error>>
        where
            A: SeqAccess<'de>,
        {
            Err(Box::new(Unexpected::SerdeUnexpected))
        }
    }

    let content = vec![Content::Bool(true), Content::U8(255)];
    let visitor = TestVisitor;

    let _ = visit_content_seq(content, visitor);
}

#[test]
fn test_visit_content_seq_with_invalid_content() {
    struct AnotherTestVisitor;

    impl<'de> Visitor<'de> for AnotherTestVisitor {
        type Value = ();
        
        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, Box<dyn de::Error>>
        where
            A: SeqAccess<'de>,
        {
            Err(Box::new(Unexpected::SerdeUnexpected))
        }
    }

    let content = vec![Content::String(String::from("invalid")), Content::None];
    let visitor = AnotherTestVisitor;

    let _ = visit_content_seq(content, visitor);
}

