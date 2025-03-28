// Answer 0

#[test]
fn test_visit_map_duplicate_end_field() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>, // Example type for the values
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::Error;

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
            let val = self.values[self.index - 1]; // Access the value corresponding to the last key
            let result: V = serde::de::Deserialize::deserialize(serde::de::value::from_value(serde_json::Value::Number(val.into())))?;
            Ok(result)
        }
    }

    let keys = vec![Field::End, Field::End]; // Duplicate "end" keys
    let values = vec![1, 2]; // Example values, only used for correct indexing

    let mock_access = MockMapAccess {
        keys,
        values,
        index: 0,
    };

    let visitor = RangeVisitor::<i32> {
        expecting: "an integer or a float",
        phantom: PhantomData,
    };

    let _result = visitor.visit_map(mock_access);
}

