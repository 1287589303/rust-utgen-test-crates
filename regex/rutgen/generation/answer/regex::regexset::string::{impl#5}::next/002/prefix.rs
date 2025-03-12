// Answer 0

#[test]
fn test_next_valid_id() {
    struct TestSetMatchesIntoIter {
        patset: PatternSet,
        it: core::ops::Range<u32>,
    }
    
    let pattern_ids: Vec<PatternID> = (0..100).map(|id| PatternID::new_unchecked(id)).collect();
    let patset = PatternSet::from(pattern_ids.iter().cloned()).unwrap();
    
    let mut iter = TestSetMatchesIntoIter {
        patset,
        it: 0..100,
    };
    
    for expected_id in 0..100 {
        let result = iter.next();
        // The assertion is omitted per instructions, but this would be where we check.
        // assert_eq!(result, Some(expected_id));
    }
}

