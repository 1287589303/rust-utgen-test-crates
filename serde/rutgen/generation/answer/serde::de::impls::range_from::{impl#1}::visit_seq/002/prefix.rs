// Answer 0

#[test]
fn test_visit_seq_with_some_value() {
    struct TestSeq {
        value: Option<i32>,
    }

    impl<'de> SeqAccess<'de> for TestSeq {
        fn next_element<T>(&mut self) -> Result<Option<T>, T::Error>
        where
            T: Deserialize<'de>,
        {
            Ok(self.value.take().map(|v| v as T))
        }
    }

    let mut seq = TestSeq { value: Some(42) };
    let visitor = RangeFromVisitor {
        expecting: "an integer",
        phantom: PhantomData,
    };
    let result: Result<i32, _> = visitor.visit_seq(&mut seq);
}

#[test]
fn test_visit_seq_with_none() {
    struct TestSeq {
        value: Option<i32>,
    }

    impl<'de> SeqAccess<'de> for TestSeq {
        fn next_element<T>(&mut self) -> Result<Option<T>, T::Error>
        where
            T: Deserialize<'de>,
        {
            Ok(None)
        }
    }

    let mut seq = TestSeq { value: None };
    let visitor = RangeFromVisitor {
        expecting: "an integer",
        phantom: PhantomData,
    };
    let result: Result<i32, _> = visitor.visit_seq(&mut seq);
}

#[test]
fn test_visit_seq_with_err() {
    struct TestSeq;

    impl<'de> SeqAccess<'de> for TestSeq {
        fn next_element<T>(&mut self) -> Result<Option<T>, T::Error>
        where
            T: Deserialize<'de>,
        {
            Err(T::Error::custom("error"))
        }
    }

    let mut seq = TestSeq;
    let visitor = RangeFromVisitor {
        expecting: "an integer",
        phantom: PhantomData,
    };
    let result: Result<i32, _> = visitor.visit_seq(&mut seq);
}

