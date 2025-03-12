// Answer 0

#[test]
fn test_next_back_none_empty_range() {
    let pattern_set = PatternSet::new_empty();
    let range = 0..0; // starting at non-positive integer and ending at zero
    let mut iter = SetMatchesIntoIter {
        patset: pattern_set,
        it: range,
    };
    let result = iter.next_back();
}

#[test]
fn test_next_back_none_negative_range() {
    let pattern_set = PatternSet::new_empty();
    let range = -1..0; // starting at a non-positive integer 
    let mut iter = SetMatchesIntoIter {
        patset: pattern_set,
        it: range,
    };
    let result = iter.next_back();
}

