// Answer 0

#[test]
fn test_visit_map_with_valid_start_end() {
    struct TestMap {
        keys: Vec<String>,
        values: Vec<i32>,
        current: usize,
    }

    impl TestMap {
        pub fn new() -> Self {
            Self { 
                keys: vec!["start".to_string(), "end".to_string()], 
                values: vec![1, 2], 
                current: 0 
            }
        }
    }

    impl MapAccess<'_> for TestMap {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.keys.len() {
                let key = &self.keys[self.current];
                self.current += 1;
                match key.as_str() {
                    "start" => Ok(Some(Field::Start)),
                    "end" => Ok(Some(Field::End)),
                    _ => Ok(None),
                }
            } else {
                Ok(None)
            }
        }

        fn next_value<R>(&mut self) -> Result<R::Value, R::Error>
        where
            R: Deserializer<'_>,
        {
            if self.current > 0 && self.current <= self.values.len() {
                Ok(self.values[self.current - 1].clone())
            } else {
                Err(serde::de::value::Error::custom("No value available"))
            }
        }
    }

    let map = TestMap::new();
    let visitor = RangeVisitor { expecting: "a range", phantom: PhantomData::<i32> };
    let _result: Result<(i32, i32), _> = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_duplicate_start() {
    struct TestMap {
        keys: Vec<String>,
        values: Vec<i32>,
        current: usize,
    }

    impl TestMap {
        pub fn new() -> Self {
            Self { 
                keys: vec!["start".to_string(), "start".to_string()], 
                values: vec![1, 2], 
                current: 0 
            }
        }
    }

    impl MapAccess<'_> for TestMap {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.keys.len() {
                let key = &self.keys[self.current];
                self.current += 1;
                match key.as_str() {
                    "start" => Ok(Some(Field::Start)),
                    _ => Ok(None),
                }
            } else {
                Ok(None)
            }
        }

        fn next_value<R>(&mut self) -> Result<R::Value, R::Error>
        where
            R: Deserializer<'_>,
        {
            if self.current > 0 && self.current <= self.values.len() {
                Ok(self.values[self.current - 1].clone())
            } else {
                Err(serde::de::value::Error::custom("No value available"))
            }
        }
    }

    let map = TestMap::new();
    let visitor = RangeVisitor { expecting: "a range", phantom: PhantomData::<i32> };
    let _result: Result<(i32, i32), _> = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_duplicate_end() {
    struct TestMap {
        keys: Vec<String>,
        values: Vec<i32>,
        current: usize,
    }

    impl TestMap {
        pub fn new() -> Self {
            Self { 
                keys: vec!["end".to_string(), "end".to_string()], 
                values: vec![2, 3], 
                current: 0 
            }
        }
    }

    impl MapAccess<'_> for TestMap {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.keys.len() {
                let key = &self.keys[self.current];
                self.current += 1;
                match key.as_str() {
                    "end" => Ok(Some(Field::End)),
                    _ => Ok(None),
                }
            } else {
                Ok(None)
            }
        }

        fn next_value<R>(&mut self) -> Result<R::Value, R::Error>
        where
            R: Deserializer<'_>,
        {
            if self.current > 0 && self.current <= self.values.len() {
                Ok(self.values[self.current - 1].clone())
            } else {
                Err(serde::de::value::Error::custom("No value available"))
            }
        }
    }

    let map = TestMap::new();
    let visitor = RangeVisitor { expecting: "a range", phantom: PhantomData::<i32> };
    let _result: Result<(i32, i32), _> = visitor.visit_map(map);
}

