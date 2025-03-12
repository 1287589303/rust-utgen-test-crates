// Answer 0

#[test]
fn test_visit_map_missing_field_tag() {
    struct MockMapAccess {
        called: bool,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = de::Error;

        fn next_key_seed<T>(
            &mut self,
            _seed: T,
        ) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.called {
                Err(de::Error::duplicate_field("tag_name"))
            } else {
                self.called = true;
                Ok(Some(TagOrContent::Tag))
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Ok(Content::Some(Box::new(Content::Unit)).into())
        }
    }

    let map_access = MockMapAccess { called: false };
    let visitor = TaggedContentVisitor::<Content>::new("tag_name");
    let result = visitor.visit_map(map_access);
}

#[test]
fn test_visit_map_no_tag_found() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        idx: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = de::Error;

        fn next_key_seed<T>(
            &mut self,
            _seed: T,
        ) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.idx < self.keys.len() {
                let key = self.keys[self.idx].clone();
                self.idx += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Ok(Content::Some(Box::new(Content::Unit)).into())
        }
    }

    let keys = vec![TagOrContent::Content(Content::Map(vec![]))];
    let map_access = MockMapAccess { keys, idx: 0 };
    let visitor = TaggedContentVisitor::<Content>::new("tag_name");

    let result = visitor.visit_map(map_access);
}

#[test]
fn test_visit_map_duplicate_tag() {
    struct MockMapAccess {
        key_calls: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = de::Error;

        fn next_key_seed<T>(
            &mut self,
            _seed: T,
        ) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.key_calls < 1 {
                self.key_calls += 1;
                Ok(Some(TagOrContent::Tag))
            } else {
                Ok(Some(TagOrContent::Tag))
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Ok(Content::Some(Box::new(Content::Unit)).into())
        }
    }

    let map_access = MockMapAccess { key_calls: 0 };
    let visitor = TaggedContentVisitor::<Content>::new("tag_name");

    let result = visitor.visit_map(map_access);
}

