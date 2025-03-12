// Answer 0

#[test]
fn test_visit_map_missing_field_empty_map() {
    struct MockMapAccess {
        keys: Vec<Option<Field>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();
        
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
            Err(())
        }
    }

    let map_access = MockMapAccess { keys: vec![], index: 0 };
    let visitor = RangeFromVisitor::<i32> { expecting: "an i32", phantom: PhantomData };
    let _ = visitor.visit_map(map_access);
}

#[test]
fn test_visit_map_missing_field_invalid_key() {
    struct MockMapAccess {
        keys: Vec<Option<Field>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();

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
            Err(())
        }
    }

    let map_access = MockMapAccess { keys: vec![Some(Field::Start), None], index: 0 };
    let visitor = RangeFromVisitor::<i32> { expecting: "an i32", phantom: PhantomData };
    let _ = visitor.visit_map(map_access);
}

