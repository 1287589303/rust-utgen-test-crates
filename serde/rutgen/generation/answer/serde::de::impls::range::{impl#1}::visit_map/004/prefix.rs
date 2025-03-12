// Answer 0

#[test]
fn test_visit_map_missing_start() {
    struct TestMapAccess {
        keys: Vec<Option<Field>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = serde::de::std::convert::Infallible;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(key)
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            // For this test, we won't actually retrieve values since we only care about missing start
            Err(serde::de::std::convert::Infallible)
        }
    }

    let test_map = TestMapAccess {
        keys: vec![], // Empty map to ensure that start is missing
        index: 0,
    };

    let visitor = RangeVisitor::<i32> {
        expecting: "an i32 range",
        phantom: std::marker::PhantomData,
    };

    let _ = visitor.visit_map(test_map);
}

