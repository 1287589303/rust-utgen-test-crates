// Answer 0

#[test]
fn test_visit_map_with_successful_entries() {
    struct TestMapAccess {
        count: usize,
    }
    
    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = ();
        
        fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error> where K: Deserialize<'de> {
            if self.count < 3 {
                self.count += 1;
                Ok(Some(IgnoredAny))
            } else {
                Ok(None)
            }
        }

        fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
        where
            K: Deserialize<'de>,
            V: Deserialize<'de>,
        {
            if self.count <= 3 {
                Ok(Some((IgnoredAny, IgnoredAny)))
            } else {
                Ok(None)
            }
        }
    }

    let visitor = InternallyTaggedUnitVisitor {
        type_name: "Test",
        variant_name: "TestVariant",
    };

    let mut access = TestMapAccess { count: 0 };
    visitor.visit_map(access).unwrap();
}

#[test]
fn test_visit_map_with_error_on_next_entry() {
    struct ErrorMapAccess {
        count: usize,
    }

    impl<'de> MapAccess<'de> for ErrorMapAccess {
        type Error = ();

        fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
        where
            K: Deserialize<'de>,
        {
            Ok(Some(IgnoredAny))
        }

        fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
        where
            K: Deserialize<'de>,
            V: Deserialize<'de>,
        {
            if self.count < 1 {
                self.count += 1;
                Err(())
            } else {
                Ok(None)
            }
        }
    }

    let visitor = InternallyTaggedUnitVisitor {
        type_name: "Test",
        variant_name: "TestVariant",
    };

    let mut access = ErrorMapAccess { count: 0 };
    let _result = visitor.visit_map(access);
}

#[test]
fn test_visit_map_with_mixed_entries() {
    struct MixedMapAccess {
        count: usize,
    }

    impl<'de> MapAccess<'de> for MixedMapAccess {
        type Error = ();

        fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
        where
            K: Deserialize<'de>,
        {
            Ok(Some(IgnoredAny))
        }

        fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
        where
            K: Deserialize<'de>,
            V: Deserialize<'de>,
        {
            if self.count == 0 {
                self.count += 1;
                Ok(Some((IgnoredAny, IgnoredAny)))
            } else {
                Err(())
            }
        }
    }

    let visitor = InternallyTaggedUnitVisitor {
        type_name: "Test",
        variant_name: "TestVariant",
    };

    let mut access = MixedMapAccess { count: 0 };
    let _result = visitor.visit_map(access);
}

