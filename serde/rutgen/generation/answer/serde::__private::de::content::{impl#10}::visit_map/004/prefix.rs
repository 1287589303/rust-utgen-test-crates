// Answer 0

#[test]
fn visit_map_with_valid_key_and_error_on_next_value() {
    struct FakeMapAccess {
        current: usize,
        keys: Vec<TagOrContent<'static>>,
    }

    impl<'de> MapAccess<'de> for FakeMapAccess {
        type Error = ();
        
        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error> {
            if self.current < self.keys.len() {
                let key = self.keys[self.current].clone();
                self.current += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error> {
            Err(()) // Simulating error on next value retrieval
        }
    }

    let map_access = FakeMapAccess {
        current: 0,
        keys: vec![
            TagOrContent::Content(Content::String("valid_key".to_string())),
        ],
    };

    let visitor = TaggedContentVisitor::<()>::new("tag_name", "expecting_string", PhantomData);

    let result: Result<_, ()> = visitor.visit_map(map_access);
    // Here, we expect `result` to be an Err due to the fake next_value method
}

#[test]
fn visit_map_with_tag_already_present() {
    struct FakeMapAccess {
        current: usize,
        keys: Vec<TagOrContent<'static>>,
    }

    impl<'de> MapAccess<'de> for FakeMapAccess {
        type Error = ();

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error> {
            if self.current < self.keys.len() {
                let key = self.keys[self.current].clone();
                self.current += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error> {
            Ok(Content::String("value".to_string())) // Simulating valid next value
        }
    }

    let map_access = FakeMapAccess {
        current: 0,
        keys: vec![
            TagOrContent::Tag,
            TagOrContent::Tag,
        ],
    };

    let visitor = TaggedContentVisitor::<()>::new("tag_name", "expecting_string", PhantomData);

    let result: Result<_, ()> = visitor.visit_map(map_access);
    // We expect that this will result in an error due to duplicate tags
}

#[test]
fn visit_map_with_missing_key() {
    struct FakeMapAccess {
        current: usize,
        keys: Vec<TagOrContent<'static>>,
    }

    impl<'de> MapAccess<'de> for FakeMapAccess {
        type Error = ();

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error> {
            if self.current < self.keys.len() {
                let key = self.keys[self.current].clone();
                self.current += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error> {
            // Simulating valid retrieval from a map that didn't populate a tag
            Ok(Content::String("value".to_string())) 
        }
    }

    let map_access = FakeMapAccess {
        current: 0,
        keys: vec![
            TagOrContent::Content(Content::String("valid_key".to_string())),
        ],
    };

    let visitor = TaggedContentVisitor::<()>::new("tag_name", "expecting_string", PhantomData);

    let result: Result<_, ()> = visitor.visit_map(map_access);
    // We expect that this will result in an error due to missing tag
}

