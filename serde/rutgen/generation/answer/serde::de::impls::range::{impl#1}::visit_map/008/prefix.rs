// Answer 0

#[test]
fn test_visit_map_duplicate_start() {
    struct TestMap {
        entries: Vec<(Field, i32)>,
        index: usize,
    }
    
    impl TestMap {
        fn new(entries: Vec<(Field, i32)>) -> Self {
            Self { entries, index: 0 }
        }
    }

    impl MapAccess<'_> for TestMap {
        type Error = serde::de::value::Error;

        fn next_key<V>(&mut self, visitor: V) -> Result<Option<Field>, Self::Error>
        where
            V: Visitor<'_>,
        {
            if self.index < self.entries.len() {
                let key = self.entries[self.index].0;
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<R>(&mut self) -> Result<i32, Self::Error>
        where
            R: Deserialize<'_>,
        {
            let value = self.entries[self.index - 1].1;
            Ok(value)
        }
    }

    let entries = vec![
        (Field::Start, 1),
        (Field::Start, 2),
    ];
    let map = TestMap::new(entries);
    let visitor = RangeVisitor::<i32> {
        expecting: "a range",
        phantom: std::marker::PhantomData,
    };

    let _result = visitor.visit_map(map);
}

