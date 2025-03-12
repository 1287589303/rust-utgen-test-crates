// Answer 0

#[test]
fn test_next_entry_with_i32_and_string() {
    struct TestMapAccess {
        entries: Vec<(i32, String)>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.current < self.entries.len() {
                let key = self.entries[self.current].0; // Assuming DeserializeSeed for i32
                self.current += 1;
                Ok(Some(key as i32))
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            if self.current > 0 {
                let value = self.entries[self.current - 1].1.clone(); // Assuming DeserializeSeed for String
                Ok(value)
            } else {
                Err(Error)
            }
        }
    }

    let mut accessor = TestMapAccess {
        entries: vec![(1, String::from("one")), (2, String::from("two"))],
        current: 0,
    };
    accessor.next_entry::<i32, String>().unwrap();
}

#[test]
fn test_next_entry_with_bool_and_i32() {
    struct TestMapAccess {
        entries: Vec<(bool, i32)>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.current < self.entries.len() {
                let key = self.entries[self.current].0; // Assuming DeserializeSeed for bool
                self.current += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            if self.current > 0 {
                let value = self.entries[self.current - 1].1; // Assuming DeserializeSeed for i32
                Ok(value)
            } else {
                Err(Error)
            }
        }
    }

    let mut accessor = TestMapAccess {
        entries: vec![(true, 100), (false, 200)],
        current: 0,
    };
    accessor.next_entry::<bool, i32>().unwrap();
}

#[test]
fn test_next_entry_with_none() {
    struct TestMapAccess {
        entries: Vec<(i32, String)>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.current < self.entries.len() {
                let key = self.entries[self.current].0;
                self.current += 1;
                Ok(Some(key as i32))
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            Err(Error) // Force an error
        }
    }

    let mut accessor = TestMapAccess {
        entries: vec![],
        current: 0,
    };
    let result = accessor.next_entry::<i32, String>();
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
} 

#[test]
#[should_panic]
fn test_next_entry_with_error() {
    struct TestMapAccess {
        entries: Vec<(i32, String)>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(Some(1))
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            Err(Error)
        }
    }

    let mut accessor = TestMapAccess {
        entries: vec![(1, String::from("one"))],
        current: 0,
    };
    accessor.next_entry::<i32, String>().unwrap();
}

