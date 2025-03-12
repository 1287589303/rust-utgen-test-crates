// Answer 0

#[test]
fn test_visit_map_multiple_tags() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error> {
            if self.current < self.keys.len() {
                let key = self.keys[self.current].clone();
                self.current += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error> {
            if self.current <= self.values.len() {
                let value = self.values[self.current - 1].clone();
                Ok(value)
            } else {
                Err(serde::de::Error::custom("No value present"))
            }
        }
    }

    let map_access = MockMapAccess {
        keys: vec![
            TagOrContent::Tag,
            TagOrContent::Tag, // Duplicate tag
            TagOrContent::Content(Content::Bool(true)),
        ],
        values: vec![
            Content::U8(1),
            Content::Str("value"),
        ],
        current: 0,
    };

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag",
        expecting: "a map with a single tag",
        value: PhantomData,
    };

    let _ = visitor.visit_map(map_access);
}

#[test]
fn test_visit_map_missing_tag() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error> {
            if self.current < self.keys.len() {
                let key = self.keys[self.current].clone();
                self.current += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error> {
            if self.current <= self.values.len() {
                let value = self.values[self.current - 1].clone();
                Ok(value)
            } else {
                Err(serde::de::Error::custom("No value present"))
            }
        }
    }

    let map_access = MockMapAccess {
        keys: vec![
            TagOrContent::Content(Content::Bool(true)), // No tag key
        ],
        values: vec![
            Content::U8(1),
        ],
        current: 0,
    };

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag",
        expecting: "a map with a single tag",
        value: PhantomData,
    };

    let _ = visitor.visit_map(map_access);
}

#[test]
fn test_visit_map_empty() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error> {
            if self.current < self.keys.len() {
                let key = self.keys[self.current].clone();
                self.current += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error> {
            Err(serde::de::Error::custom("No value present"))
        }
    }

    let map_access = MockMapAccess {
        keys: vec![],
        current: 0,
    };

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag",
        expecting: "a map with a single tag",
        value: PhantomData,
    };

    let _ = visitor.visit_map(map_access);
}

