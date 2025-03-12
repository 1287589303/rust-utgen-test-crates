// Answer 0

#[test]
fn test_visit_map_with_valid_data() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();
        
        fn next_key_seed<K>(self: &mut Self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }
        
        fn next_value<V>(self: &mut Self) -> Result<Content<'de>, Self::Error>
        where
            V: Deserialize<'de>,
        {
            let value = self.values.remove(0);
            Ok(value)
        }
    }

    let map = MockMapAccess {
        keys: vec![
            TagOrContent::Tag,
            TagOrContent::Content(Content::Bool(true)),
        ],
        values: vec![
            Content::Bool(true),
        ],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag_name",
        expecting: "expecting",
        value: PhantomData,
    };

    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_duplicate_tag() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();

        fn next_key_seed<K>(self: &mut Self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }
        
        fn next_value<V>(self: &mut Self) -> Result<Content<'de>, Self::Error>
        where
            V: Deserialize<'de>,
        {
            let value = self.values.remove(0);
            Ok(value)
        }
    }

    let map = MockMapAccess {
        keys: vec![
            TagOrContent::Tag,
            TagOrContent::Tag,
            TagOrContent::Content(Content::Bool(true)),
        ],
        values: vec![
            Content::Bool(true),
            Content::Bool(true),
        ],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag_name",
        expecting: "expecting",
        value: PhantomData,
    };

    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_missing_tag() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();

        fn next_key_seed<K>(self: &mut Self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(self: &mut Self) -> Result<Content<'de>, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(())
        }
    }

    let map = MockMapAccess {
        keys: vec![
            TagOrContent::Content(Content::Bool(true)),
        ],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag_name",
        expecting: "expecting",
        value: PhantomData,
    };

    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_malformed_data() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();

        fn next_key_seed<K>(self: &mut Self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(self: &mut Self) -> Result<Content<'de>, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(())
        }
    }

    let map = MockMapAccess {
        keys: vec![
            TagOrContent::Tag,
            TagOrContent::Content(Content::Bool(true)),
        ],
        values: vec![],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag_name",
        expecting: "expecting",
        value: PhantomData,
    };

    let _ = visitor.visit_map(map);
}

