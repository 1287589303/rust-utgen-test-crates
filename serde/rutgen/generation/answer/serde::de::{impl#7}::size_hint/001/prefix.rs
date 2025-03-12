// Answer 0

#[test]
fn test_size_hint_none() {
    struct TestMapAccess;

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = ();
        
        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            Err(())
        }

        fn size_hint(&self) -> Option<usize> {
            None
        }
    }

    let mut test_map = TestMapAccess;
    let _ = test_map.size_hint();
}

#[test]
fn test_size_hint_some_zero() {
    struct TestMapAccess;

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = ();
        
        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            Err(())
        }

        fn size_hint(&self) -> Option<usize> {
            Some(0)
        }
    }

    let mut test_map = TestMapAccess;
    let _ = test_map.size_hint();
}

#[test]
fn test_size_hint_some_non_negative() {
    struct TestMapAccess;

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = ();
        
        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            Err(())
        }

        fn size_hint(&self) -> Option<usize> {
            Some(5)
        }
    }

    let mut test_map = TestMapAccess;
    let _ = test_map.size_hint();
}

