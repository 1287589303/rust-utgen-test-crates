// Answer 0

#[test]
fn test_visit_map_err_on_next_key() {
    struct MockMapAccess {
        called_next_key: bool,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::Error;

        fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
        where
            K: serde::de::Deserialize<'de>,
        {
            if self.called_next_key {
                Err(serde::de::Error::custom("key error"))
            } else {
                self.called_next_key = true;
                Ok(Some(Field::End)) // to simulate first call returning Field::End
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: serde::de::Deserialize<'de>,
        {
            Err(serde::de::Error::custom("value error"))
        }
    }

    let map_access = MockMapAccess {
        called_next_key: false,
    };
    let visitor = RangeVisitor::<i32> {
        expecting: "an integer range",
        phantom: std::marker::PhantomData,
    };

    let _ = visitor.visit_map(map_access);
}

