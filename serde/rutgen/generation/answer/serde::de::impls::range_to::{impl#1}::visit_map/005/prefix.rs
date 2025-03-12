// Answer 0

#[test]
fn test_visit_map_with_invalid_value_for_end() {
    struct TestMapAccess {
        key: Option<Field>,
        value: Option<i64>, // assuming i64 is invalid for Idx
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = serde::de::value::Error;

        fn next_key<'a>(&mut self) -> Result<Option<Field>, Self::Error> {
            Ok(self.key.take())
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            match self.value.take() {
                Some(_) => Err(serde::de::value::Error::custom("invalid value type")),
                None => Err(serde::de::value::Error::custom("no value")),
            }
        }
    }

    let mut map = TestMapAccess {
        key: Some(Field::End),
        value: Some(42), // Invalid type for Idx
    };

    let visitor = RangeToVisitor::<i64> {
        expecting: "an i64 integer",
        phantom: std::marker::PhantomData,
    };

    let result = visitor.visit_map(&mut map);
}

#[test]
fn test_visit_map_with_duplicate_end_field() {
    struct TestMapAccess {
        keys: Vec<Option<Field>>,
        values: Vec<Option<i64>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = serde::de::value::Error;

        fn next_key<'a>(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].take();
                self.index += 1;
                Ok(key)
            } else {
                Ok(None)
            }
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index > 0 {
                if let Some(_) = self.values[self.index - 1] {
                    return Err(serde::de::value::Error::custom("duplicate field"));
                }
            }
            Err(serde::de::value::Error::custom("invalid value type"))
        }
    }

    let mut map = TestMapAccess {
        keys: vec![Some(Field::End), Some(Field::End)],
        values: vec![Some(42), Some(43)], // Values won't be used since we simulate duplicate
        index: 0,
    };

    let visitor = RangeToVisitor::<i64> {
        expecting: "an i64 integer",
        phantom: std::marker::PhantomData,
    };

    let result = visitor.visit_map(&mut map);
}

