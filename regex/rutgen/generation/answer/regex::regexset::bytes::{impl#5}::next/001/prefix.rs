// Answer 0

#[test]
fn test_next_with_empty_range() {
    let patset = PatternSet::new(vec![]).unwrap();
    let it = 0..0; // Empty range
    let mut iter = SetMatchesIntoIter { patset, it };
    let result = iter.next();
}

#[test]
fn test_next_with_exhausted_range() {
    let patset = PatternSet::new(vec![meta::Regex::new("a").unwrap()]).unwrap();
    let it = 1..1; // Exhausted range
    let mut iter = SetMatchesIntoIter { patset, it };
    let result = iter.next();
}

