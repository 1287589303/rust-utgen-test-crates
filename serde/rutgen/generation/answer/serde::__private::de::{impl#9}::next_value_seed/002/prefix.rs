// Answer 0

#[test]
fn test_next_value_seed_none_pending_content() {
    struct TestError;
    impl Error for TestError {
        fn custom<T>(_: T) -> Self {
            TestError
        }
    }

    struct EmptySeed;

    impl<'de> DeserializeSeed<'de> for EmptySeed {
        type Value = ();
        fn deserialize<T>(self, _: T) -> Result<Self::Value, TestError> {
            Ok(())
        }
    }

    let mut map_access = FlatMapAccess {
        iter: &mut [].iter(),
        pending_content: None,
        _marker: PhantomData::<TestError>,
    };
    let seed = EmptySeed;

    let _result = map_access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_none_pending_content_another_dissolving() {
    struct TestError;
    impl Error for TestError {
        fn custom<T>(_: T) -> Self {
            TestError
        }
    }

    struct AlwaysEmptySeed;

    impl<'de> DeserializeSeed<'de> for AlwaysEmptySeed {
        type Value = ();
        fn deserialize<T>(self, _: T) -> Result<Self::Value, TestError> {
            Ok(())
        }
    }

    let iter: &[Option<(Content, Content)>] = &[];
    let mut map_access = FlatMapAccess {
        iter: iter.iter(),
        pending_content: None,
        _marker: PhantomData::<TestError>,
    };

    let seed = AlwaysEmptySeed;

    let _result = map_access.next_value_seed(seed);
}

