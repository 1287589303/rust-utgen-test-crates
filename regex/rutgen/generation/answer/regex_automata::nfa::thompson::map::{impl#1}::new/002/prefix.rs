// Answer 0

#[test]
#[should_panic]
fn test_utf8_suffix_map_new_zero_capacity() {
    let _map = Utf8SuffixMap::new(0);
}

#[test]
#[should_panic]
fn test_utf8_suffix_map_new_negative_capacity() {
    let _map = Utf8SuffixMap::new(!0); // Using a negative capacity by inverting 0
}

