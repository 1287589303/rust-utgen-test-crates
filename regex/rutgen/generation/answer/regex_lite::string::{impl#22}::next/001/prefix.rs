// Answer 0

#[test]
fn test_next_with_valid_match() {
    struct FakeFinder<'r, 'h> {
        matches: Vec<Match<'h>>,
        index: usize,
    }
    
    impl<'r, 'h> Iterator for FakeFinder<'r, 'h> {
        type Item = Match<'h>;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.matches.len() {
                let result = self.matches[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let haystack = "hello world";
    let match1 = Match::new(haystack, 0, 5);
    let match2 = Match::new(haystack, 6, 11);
    let finder = FakeFinder {
        matches: vec![match1, match2],
        index: 0,
    };

    let mut split = Split {
        haystack,
        finder,
        last: 0,
    };

    let result = split.next();
    let expected_range = 0..5;
    assert!(result.is_some());
    assert_eq!(&haystack[expected_range.clone()], result.unwrap());
}

#[test]
fn test_next_with_multiple_valid_matches() {
    struct FakeFinder<'r, 'h> {
        matches: Vec<Match<'h>>,
        index: usize,
    }
    
    impl<'r, 'h> Iterator for FakeFinder<'r, 'h> {
        type Item = Match<'h>;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.matches.len() {
                let result = self.matches[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let haystack = "hello world";
    let match1 = Match::new(haystack, 0, 5);
    let match2 = Match::new(haystack, 6, 11);
    let finder = FakeFinder {
        matches: vec![match1, match2],
        index: 0,
    };

    let mut split = Split {
        haystack,
        finder,
        last: 0,
    };

    let result1 = split.next();
    let expected_range1 = 0..5;
    assert!(result1.is_some());
    assert_eq!(&haystack[expected_range1.clone()], result1.unwrap());

    let result2 = split.next();
    let expected_range2 = 5..6;
    assert!(result2.is_some());
    assert_eq!(&haystack[expected_range2.clone()], result2.unwrap());
}

#[test]
fn test_next_with_last_valid_index() {
    struct FakeFinder<'r, 'h> {
        matches: Vec<Match<'h>>,
        index: usize,
    }
    
    impl<'r, 'h> Iterator for FakeFinder<'r, 'h> {
        type Item = Match<'h>;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.matches.len() {
                let result = self.matches[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let haystack = "test case example";
    let match1 = Match::new(haystack, 0, 4);
    let match2 = Match::new(haystack, 5, 9);
    let finder = FakeFinder {
        matches: vec![match1, match2],
        index: 0,
    };

    let mut split = Split {
        haystack,
        finder,
        last: 4,
    };

    let result1 = split.next();
    let expected_range1 = 4..5;
    assert!(result1.is_some());
    assert_eq!(&haystack[expected_range1.clone()], result1.unwrap());
}

