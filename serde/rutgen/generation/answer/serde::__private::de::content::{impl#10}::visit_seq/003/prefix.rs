// Answer 0

#[test]
fn test_visit_seq_with_valid_tag_and_rest() {
    struct MockSeqAccess {
        values: Vec<Content>,
        current_index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = ();

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.current_index < self.values.len() {
                let value = self.values[self.current_index].clone();
                self.current_index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    let tag = Content::String("tag_value".to_string());
    let rest_values = vec![Content::I32(42), Content::Bool(true)];
    let seq_access = MockSeqAccess {
        values: vec![tag.clone()].into_iter().chain(rest_values.clone()).collect(),
        current_index: 0,
    };
    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag_name",
        expecting: "expecting value",
        value: PhantomData,
    };
    let _: Result<(Content, Content), ()> = visitor.visit_seq(seq_access);
}

#[test]
fn test_visit_seq_with_empty_seq() {
    struct EmptySeqAccess;

    impl<'de> SeqAccess<'de> for EmptySeqAccess {
        type Error = ();

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Ok(None)
        }
    }

    let seq_access = EmptySeqAccess;
    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag_name",
        expecting: "expecting value",
        value: PhantomData,
    };
    let _: Result<(Content, Content), ()> = visitor.visit_seq(seq_access);
}

#[test]
fn test_visit_seq_with_err_on_next_element() {
    struct ErrSeqAccess;

    impl<'de> SeqAccess<'de> for ErrSeqAccess {
        type Error = ();

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Err(())
        }
    }

    let seq_access = ErrSeqAccess;
    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag_name",
        expecting: "expecting value",
        value: PhantomData,
    };
    let _: Result<(Content, Content), ()> = visitor.visit_seq(seq_access);
}

