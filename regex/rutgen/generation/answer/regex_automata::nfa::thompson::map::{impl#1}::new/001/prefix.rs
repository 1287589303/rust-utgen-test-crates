// Answer 0

#[test]
fn test_new_utf8_suffix_map_capacity_1() {
    let capacity = 1;
    let map = Utf8SuffixMap::new(capacity);
}

#[test]
fn test_new_utf8_suffix_map_capacity_10() {
    let capacity = 10;
    let map = Utf8SuffixMap::new(capacity);
}

#[test]
fn test_new_utf8_suffix_map_capacity_100() {
    let capacity = 100;
    let map = Utf8SuffixMap::new(capacity);
}

#[test]
fn test_new_utf8_suffix_map_capacity_1000() {
    let capacity = 1000;
    let map = Utf8SuffixMap::new(capacity);
}

#[test]
fn test_new_utf8_suffix_map_capacity_10000() {
    let capacity = 10000;
    let map = Utf8SuffixMap::new(capacity);
}

