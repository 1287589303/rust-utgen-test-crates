// Answer 0

#[test]
fn test_visit_map_with_valid_tag_and_content_pair() {
    struct MockMap {
        data: Vec<(TagOrContent, Content)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = ();

        fn next_key_seed<T>(&mut self, visitor: T) -> Result<Option<TagOrContent>, Self::Error>
        where
            T: DeserializeSeed<'de, Value = TagOrContent>,
        {
            if self.index < self.data.len() {
                let (key, _) = &self.data[self.index];
                self.index += 1;
                Ok(Some(key.clone()))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            if let Some((_, value)) = self.data.get(self.index - 1) {
                self.index += 1; 
                value.deserialize(visitor)
            } else {
                Err(())
            }
        }
    }

    let tag_content_pairs = vec![
        (TagOrContent::Tag, Content::Bool(true)),
        (TagOrContent::Content(Content::String("example".to_string())), Content::String("value".to_string())),
    ];

    let mut mock_map = MockMap { data: tag_content_pairs, index: 0 };
    let visitor = TaggedContentVisitor::<T> { tag_name: "tag", expecting: "expected value", value: PhantomData };

    let _ = visitor.visit_map(&mut mock_map);
}

#[test]
#[should_panic]
fn test_visit_map_with_duplicate_tag() {
    struct MockMap {
        data: Vec<(TagOrContent, Content)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = ();

        fn next_key_seed<T>(&mut self, visitor: T) -> Result<Option<TagOrContent>, Self::Error>
        where
            T: DeserializeSeed<'de, Value = TagOrContent>,
        {
            if self.index < self.data.len() {
                let (key, _) = &self.data[self.index];
                self.index += 1;
                Ok(Some(key.clone()))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            if let Some((_, value)) = self.data.get(self.index - 1) {
                self.index += 1; 
                value.deserialize(visitor)
            } else {
                Err(())
            }
        }
    }

    let tag_content_pairs = vec![
        (TagOrContent::Tag, Content::Bool(true)),
        (TagOrContent::Tag, Content::Bool(false)), // Duplicate tag
    ];

    let mut mock_map = MockMap { data: tag_content_pairs, index: 0 };
    let visitor = TaggedContentVisitor::<T> { tag_name: "tag", expecting: "expected value", value: PhantomData };

    let _ = visitor.visit_map(&mut mock_map);
}

#[test]
fn test_visit_map_with_missing_tag() {
    struct MockMap {
        data: Vec<(TagOrContent, Content)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = ();

        fn next_key_seed<T>(&mut self, visitor: T) -> Result<Option<TagOrContent>, Self::Error>
        where
            T: DeserializeSeed<'de, Value = TagOrContent>,
        {
            if self.index < self.data.len() {
                let (key, _) = &self.data[self.index];
                self.index += 1;
                Ok(Some(key.clone()))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            if let Some((_, value)) = self.data.get(self.index - 1) {
                self.index += 1; 
                value.deserialize(visitor)
            } else {
                Err(())
            }
        }
    }

    let tag_content_pairs = vec![
        (TagOrContent::Content(Content::String("example".to_string())), Content::String("value".to_string())), // No tag
    ];

    let mut mock_map = MockMap { data: tag_content_pairs, index: 0 };
    let visitor = TaggedContentVisitor::<T> { tag_name: "tag", expecting: "expected value", value: PhantomData };

    let _ = visitor.visit_map(&mut mock_map);
}

