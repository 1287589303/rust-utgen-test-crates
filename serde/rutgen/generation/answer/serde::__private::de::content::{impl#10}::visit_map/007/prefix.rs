// Answer 0

#[test]
fn test_visit_map_with_duplicate_tag() {
    struct MockMapAccess {
        called: bool,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = crate::Error;

        fn next_key_seed<K>(
            &mut self,
            _: K,
        ) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: serde::de::DeserializeSeed<'de>,
        {
            if self.called {
                Ok(Some(TagOrContent::Tag))
            } else {
                self.called = true;
                Ok(Some(TagOrContent::Tag))
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: serde::de::Deserialize<'de>,
        {
            Err(crate::Error::duplicate_field("tag_name"))
        }
    }

    let mut map_access = MockMapAccess { called: false };
    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag_name",
        expecting: "Expecting a tag",
        value: PhantomData,
    };

    let _ = visitor.visit_map(&mut map_access);
}

#[test]
fn test_visit_map_with_missing_key() {
    struct MockMapAccess {
        called: bool,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = crate::Error;

        fn next_key_seed<K>(
            &mut self,
            _: K,
        ) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: serde::de::DeserializeSeed<'de>,
        {
            if self.called {
                Ok(None)
            } else {
                self.called = true;
                Ok(Some(TagOrContent::Tag))
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: serde::de::Deserialize<'de>,
        {
            Err(crate::Error::missing_field("tag_name"))
        }
    }

    let mut map_access = MockMapAccess { called: false };
    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag_name",
        expecting: "Expecting a tag",
        value: PhantomData,
    };

    let _ = visitor.visit_map(&mut map_access);
}

