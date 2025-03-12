// Answer 0

#[test]
fn test_visit_map_start_error() {
    struct FakeMap {
        called_next_key: bool,
        called_next_value: bool,
    }

    impl<'de> MapAccess<'de> for FakeMap {
        type Error = serde::de::value::Error;

        fn next_key<T>(&mut self) -> Result<Option<Field>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if !self.called_next_key {
                self.called_next_key = true;
                Ok(Some(Field::Start))
            } else {
                Ok(None)
            }
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if !self.called_next_value {
                self.called_next_value = true;
                Err(serde::de::value::Error::custom("error in next_value"))
            } else {
                unreachable!()
            }
        }
    }

    let map = FakeMap {
        called_next_key: false,
        called_next_value: false,
    };

    let visitor = RangeVisitor {
        expecting: "a range",
        phantom: std::marker::PhantomData,
    };

    let result: Result<(i32, i32), _> = visitor.visit_map(map);
}

