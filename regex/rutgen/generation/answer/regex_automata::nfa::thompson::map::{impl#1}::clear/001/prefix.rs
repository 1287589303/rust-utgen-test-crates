// Answer 0

#[test]
fn test_clear_map_when_empty() {
    let capacity = 10;
    let mut map = Utf8SuffixMap::new(capacity);
    map.clear();
}

#[test]
fn test_clear_map_when_empty_capacity_zero() {
    let capacity = 0;
    let mut map = Utf8SuffixMap::new(capacity);
    map.clear();
}

