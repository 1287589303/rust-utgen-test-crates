// Answer 0

#[test]
fn test_visit_map_with_valid_start_key_and_error_next_key() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].clone();
                self.index += 1;
                Ok(Some(key))
            } else {
                Err(serde::de::value::Error::custom("next_key error"))
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            if self.index - 1 < self.values.len() {
                Ok(self.values[self.index - 1].clone() as V)
            } else {
                Err(serde::de::value::Error::custom("value error"))
            }
        }
    }

    let map = MockMapAccess {
        keys: vec![Field::Start],
        values: vec![42],
        index: 0,
    };

    let visitor = RangeVisitor::<i32> {
        expecting: "a range",
        phantom: std::marker::PhantomData,
    };

    let result: Result<(i32, i32), _> = visitor.visit_map(map);
} 

#[test]
fn test_visit_map_with_duplicate_start_key() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].clone();
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
            if self.index - 1 < self.values.len() {
                Ok(self.values[self.index - 1].clone() as V)
            } else {
                Err(serde::de::value::Error::custom("value error"))
            }
        }
    }

    let map = MockMapAccess {
        keys: vec![Field::Start, Field::Start],
        values: vec![42, 43],
        index: 0,
    };

    let visitor = RangeVisitor::<i32> {
        expecting: "a range",
        phantom: std::marker::PhantomData,
    };

    let result: Result<(i32, i32), _> = visitor.visit_map(map);
} 

#[test]
fn test_visit_map_with_missing_end_field() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].clone();
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
            if self.index - 1 < self.values.len() {
                Ok(self.values[self.index - 1].clone() as V)
            } else {
                Err(serde::de::value::Error::custom("value error"))
            }
        }
    }

    let map = MockMapAccess {
        keys: vec![Field::Start], // Missing Field::End
        values: vec![42],
        index: 0,
    };

    let visitor = RangeVisitor::<i32> {
        expecting: "a range",
        phantom: std::marker::PhantomData,
    };

    let result: Result<(i32, i32), _> = visitor.visit_map(map);
}

