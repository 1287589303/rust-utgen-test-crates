// Answer 0

#[test]
fn test_size_hint_empty() {
    struct TestCapturesPatternIter<'c> {
        size: usize,
    }
    
    impl<'c> captures::CapturesPatternIter<'c> {
        fn new_empty() -> Self {
            TestCapturesPatternIter { size: 0 }
        }
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.size, None)
        }
    }

    let haystack: &[u8] = b"";
    let it = TestCapturesPatternIter::new_empty();
    let captures = SubCaptureMatches { haystack, it };

    captures.size_hint();
}

#[test]
fn test_size_hint_one() {
    struct TestCapturesPatternIter<'c> {
        size: usize,
    }
    
    impl<'c> captures::CapturesPatternIter<'c> {
        fn new_one() -> Self {
            TestCapturesPatternIter { size: 1 }
        }
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.size, Some(self.size))
        }
    }

    let haystack: &[u8] = b"example";
    let it = TestCapturesPatternIter::new_one();
    let captures = SubCaptureMatches { haystack, it };

    captures.size_hint();
}

#[test]
fn test_size_hint_multiple() {
    struct TestCapturesPatternIter<'c> {
        size: usize,
    }
    
    impl<'c> captures::CapturesPatternIter<'c> {
        fn new_multiple(size: usize) -> Self {
            TestCapturesPatternIter { size }
        }
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.size, Some(self.size))
        }
    }

    let haystack: &[u8] = b"sample string";
    let it = TestCapturesPatternIter::new_multiple(3);
    let captures = SubCaptureMatches { haystack, it };

    captures.size_hint();
}

