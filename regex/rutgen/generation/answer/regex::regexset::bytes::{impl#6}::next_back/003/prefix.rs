// Answer 0

#[test]
fn test_next_back_some_non_matching() {
    let pattern_set = PatternSet::build().add(PatternID::new_unchecked(0)).finish();
    let range = 5..10; // Valid range to include Some values
    let mut iter = SetMatchesIntoIter { patset: pattern_set, it: range };

    let _ = iter.next_back(); // This should return Some(9)
    let _ = iter.next_back(); // This should return Some(8)
    let _ = iter.next_back(); // This should return Some(7)
}

#[test]
fn test_next_back_end_of_range() {
    let pattern_set = PatternSet::build().add(PatternID::new_unchecked(0)).finish();
    let range = 5..=5; // Valid range with only one element
    let mut iter = SetMatchesIntoIter { patset: pattern_set, it: range };

    let _ = iter.next_back(); // This should return Some(5)
    let _ = iter.next_back(); // This should return None
}

#[test]
fn test_next_back_none() {
    let pattern_set = PatternSet::build().finish(); // No patterns in the set
    let range = 0..0; // Empty range
    let mut iter = SetMatchesIntoIter { patset: pattern_set, it: range };

    let result = iter.next_back(); // This should return None
}

