// Answer 0

#[test]
fn test_visit_map_valid_case() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            let value = self.values.remove(0);
            Ok(value as V)
        }
    }

    let keys = vec![Field::End];
    let values = vec![42];
    let map_access = MockMapAccess { keys, values, index: 0 };
    let visitor = RangeToVisitor::<i32> { expecting: "an i32", phantom: std::marker::PhantomData };

    let _ = visitor.visit_map(map_access);
}

#[test]
fn test_visit_map_multiple_keys() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            let value = self.values.remove(0);
            Ok(value as V)
        }
    }

    let keys = vec![Field::End];
    let values = vec![7];
    let map_access = MockMapAccess { keys, values, index: 0 };
    let visitor = RangeToVisitor::<i32> { expecting: "an i32", phantom: std::marker::PhantomData };

    let _ = visitor.visit_map(map_access);
}

#[test]
fn test_visit_map_duplicate_field_error() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            let value = self.values.remove(0);
            Ok(value as V)
        }
    }

    let keys = vec![Field::End, Field::End];  // Duplicate 'end' key
    let values = vec![1, 2];
    let map_access = MockMapAccess { keys, values, index: 0 };
    let visitor = RangeToVisitor::<i32> { expecting: "an i32", phantom: std::marker::PhantomData };

    let result = visitor.visit_map(map_access);
    let _ = result.expect_err("Expected duplicate field error");
}

#[test]
fn test_visit_map_missing_end_field_error() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            let value = self.values.remove(0);
            Ok(value as V)
        }
    }

    let keys = vec![];  // No keys
    let values = vec![];
    let map_access = MockMapAccess { keys, values, index: 0 };
    let visitor = RangeToVisitor::<i32> { expecting: "an i32", phantom: std::marker::PhantomData };

    let result = visitor.visit_map(map_access);
    let _ = result.expect_err("Expected missing field error");
}

