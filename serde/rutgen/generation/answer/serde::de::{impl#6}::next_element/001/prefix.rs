// Answer 0

#[test]
fn test_next_element_with_empty_seq_access() {
    struct EmptySeqAccess;
    impl<'de> SeqAccess<'de> for EmptySeqAccess {
        type Error = Error;
        fn next_element_seed<T>(&mut self, _: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            Ok(None)
        }
    }
    let mut seq_access = EmptySeqAccess;
    let _ = seq_access.next_element::<T>();
}

#[test]
fn test_next_element_with_single_element_seq_access() {
    struct SingleElementSeqAccess {
        called: bool,
    }
    impl<'de> SeqAccess<'de> for SingleElementSeqAccess {
        type Error = Error;
        fn next_element_seed<T>(&mut self, _: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            self.called = true;
            Ok(Some(T::deserialize(&mut DeserializerMock)?))
        }
    }
    let mut seq_access = SingleElementSeqAccess { called: false };
    let _ = seq_access.next_element::<T>();
    assert!(seq_access.called);
}

#[test]
fn test_next_element_with_error_seq_access() {
    struct ErrorSeqAccess;
    impl<'de> SeqAccess<'de> for ErrorSeqAccess {
        type Error = Error;
        fn next_element_seed<T>(&mut self, _: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            Err(Error {})
        }
    }
    let mut seq_access = ErrorSeqAccess;
    let result = seq_access.next_element::<T>();
    assert!(result.is_err());
}

