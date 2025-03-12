// Answer 0

#[test]
fn test_visit_seq_with_ok_and_err() {
    struct TestSeqAccess {
        calls: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeqAccess {
        type Error = serde::de::value::Error;

        fn next_element(&mut self) -> Result<Option<Content<'de>>, Self::Error> {
            if self.calls == 0 {
                self.calls += 1;
                Ok(Some(Content::Bool(true))) // First call returns Ok(Some(value))
            } else {
                Err(serde::de::value::Error::custom("Expected error")) // Second call returns Err(error)
            }
        }

        fn size_hint(&self) -> Option<usize> {
            Some(2) // Indicate that we expect two elements in total
        }
    }

    let visitor = TestSeqAccess { calls: 0 };
    let content_visitor = ContentVisitor { value: PhantomData };
    let _ = content_visitor.visit_seq(visitor);
}

#[test]
fn test_visit_seq_with_empty_sequence() {
    struct EmptySeqAccess;

    impl<'de> SeqAccess<'de> for EmptySeqAccess {
        type Error = serde::de::value::Error;

        fn next_element(&mut self) -> Result<Option<Content<'de>>, Self::Error> {
            Ok(None) // Returns None for an empty sequence
        }

        fn size_hint(&self) -> Option<usize> {
            Some(0) // No elements in the sequence
        }
    }

    let visitor = EmptySeqAccess;
    let content_visitor = ContentVisitor { value: PhantomData };
    let _ = content_visitor.visit_seq(visitor);
}

#[test]
fn test_visit_seq_with_multiple_elements() {
    struct MultiSeqAccess {
        calls: usize,
    }

    impl<'de> SeqAccess<'de> for MultiSeqAccess {
        type Error = serde::de::value::Error;

        fn next_element(&mut self) -> Result<Option<Content<'de>>, Self::Error> {
            match self.calls {
                0 => {
                    self.calls += 1;
                    Ok(Some(Content::U8(255))) // First call returns Ok(Some(value))
                }
                1 => {
                    self.calls += 1;
                    Ok(Some(Content::I32(-1))) // Second call also returns Ok(Some(value))
                }
                _ => Err(serde::de::value::Error::custom("Expected error after two valid elements")), // Following calls return Err(error)
            }
        }

        fn size_hint(&self) -> Option<usize> {
            Some(3) // Indicate that there are three elements but an error will occur
        }
    }

    let visitor = MultiSeqAccess { calls: 0 };
    let content_visitor = ContentVisitor { value: PhantomData };
    let _ = content_visitor.visit_seq(visitor);
}

