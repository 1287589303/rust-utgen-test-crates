// Answer 0

#[test]
fn test_visit_content_seq_err_on_end() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();
        fn visit_seq<V>(self, _seq: &mut V) -> Result<Self::Value, E>
        where
            V: SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let content = vec![
        Content::U8(1),
        Content::Seq(vec![
            Content::U16(2),
            Content::U32(3),
        ]),
        Content::U64(4),
    ];

    let visitor = VisitorImpl;
    let result = visit_content_seq(content, visitor);
}

#[test]
fn test_visit_content_seq_empty_content() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();
        fn visit_seq<V>(self, _seq: &mut V) -> Result<Self::Value, E>
        where
            V: SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let content = vec![];

    let visitor = VisitorImpl;
    let result = visit_content_seq(content, visitor);
}

#[test]
fn test_visit_content_seq_with_invalid_seq() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();
        fn visit_seq<V>(self, _seq: &mut V) -> Result<Self::Value, E>
        where
            V: SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let content = vec![
        Content::Seq(vec![
            Content::U8(1),
            Content::F32(3.14),
        ]),
        Content::String(String::from("test")),
        Content::None,
    ];

    let visitor = VisitorImpl;
    let result = visit_content_seq(content, visitor);
}

