pub fn new(capacity: usize) -> PatternSet {
        assert!(
            capacity <= PatternID::LIMIT,
            "pattern set capacity exceeds limit of {}",
            PatternID::LIMIT,
        );
        PatternSet {
            len: 0,
            which: alloc::vec![false; capacity].into_boxed_slice(),
        }
    }