// Answer 0

#[test]
fn test_next_returns_some_when_contains() {
    struct DummyPatternSet {
        ids: Vec<usize>,
    }

    impl PatternSet for DummyPatternSet {
        fn contains(&self, id: PatternID) -> bool {
            self.ids.contains(&(id.0 as usize))
        }
    }

    let mut it = 0..5; // an iterator that produces 0 to 4
    let patset = DummyPatternSet { ids: vec![0, 1, 2] }; // contains 0, 1, 2

    let mut iter = SetMatchesIntoIter { patset, it };

    let result = iter.next();
}  

#[test]
fn test_next_returns_some_with_boundary_id() {
    struct DummyPatternSet {
        ids: Vec<usize>,
    }

    impl PatternSet for DummyPatternSet {
        fn contains(&self, id: PatternID) -> bool {
            self.ids.contains(&(id.0 as usize))
        }
    }

    let mut it = 3..6; // an iterator that produces 3 to 5
    let patset = DummyPatternSet { ids: vec![3, 4, 5] }; // contains 3, 4, 5

    let mut iter = SetMatchesIntoIter { patset, it };

    let result = iter.next();
}  

#[test]
fn test_next_with_single_element_pattern_set() {
    struct DummyPatternSet {
        ids: Vec<usize>,
    }

    impl PatternSet for DummyPatternSet {
        fn contains(&self, id: PatternID) -> bool {
            self.ids.contains(&(id.0 as usize))
        }
    }

    let mut it = 0..1; // an iterator that produces only 0
    let patset = DummyPatternSet { ids: vec![0] }; // contains only 0

    let mut iter = SetMatchesIntoIter { patset, it };

    let result = iter.next();
}  

#[test]
fn test_next_with_multiple_ids_trying_non_matching() {
    struct DummyPatternSet {
        ids: Vec<usize>,
    }

    impl PatternSet for DummyPatternSet {
        fn contains(&self, id: PatternID) -> bool {
            self.ids.contains(&(id.0 as usize))
        }
    }

    let mut it = 2..5; // an iterator that produces 2 to 4
    let patset = DummyPatternSet { ids: vec![0, 1, 4] }; // contains 0, 1, 4

    let mut iter = SetMatchesIntoIter { patset, it };

    let result = iter.next();
}  

#[test]
fn test_next_with_exceeding_iterator() {
    struct DummyPatternSet {
        ids: Vec<usize>,
    }

    impl PatternSet for DummyPatternSet {
        fn contains(&self, id: PatternID) -> bool {
            self.ids.contains(&(id.0 as usize))
        }
    }

    let mut it = 5..10; // an iterator that produces elements 5 to 9
    let patset = DummyPatternSet { ids: vec![0, 1, 2, 3, 4] }; // does not contain 5, 6, 7, 8, or 9

    let mut iter = SetMatchesIntoIter { patset, it };

    let result = iter.next();
}  

