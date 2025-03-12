// Answer 0

#[test]
fn test_visit_map_duplicate_tag() {
    struct MockMap {
        entries: Vec<(TagOrContent<'static>, Content)>,
        index: usize,
    }
    
    impl<'de> MapAccess<'de> for MockMap {
        type Error = de::Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.index < self.entries.len() {
                let key = self.entries[self.index].0.clone();
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            if self.index < self.entries.len() {
                let value = self.entries[self.index].1.clone();
                self.index += 1;
                Ok(value)
            } else {
                Err(de::Error::custom("No more values"))
            }
        }
    }

    let map_content = vec![
        (TagOrContent::Tag, Content::String("tag_value_1".to_string())),
        (TagOrContent::Tag, Content::String("tag_value_2".to_string())), // Duplicate tag
    ];
    
    let mut map = MockMap {
        entries: map_content,
        index: 0,
    };

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag_name",
        expecting: "Expected tag",
        value: PhantomData,
    };

    let result = visitor.visit_map(&mut map);
}

