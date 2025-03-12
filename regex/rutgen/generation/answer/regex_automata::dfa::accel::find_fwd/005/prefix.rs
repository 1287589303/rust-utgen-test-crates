// Answer 0

#[test]
fn test_find_fwd_two_needles_none_found() {
    let needles = [0x00, 0x01];
    let haystack = [0x02, 0x03, 0x04];
    let at = 0;
    let result = find_fwd(&needles, &haystack, at);
}

#[test]
#[should_panic(expected = "invalid needles length: 4")]
fn test_find_fwd_invalid_length_panic() {
    let needles = [0x00, 0x01, 0x02, 0x03];
    let haystack = [0x02, 0x03, 0x04];
    let at = 0;
    let result = find_fwd(&needles, &haystack, at);
}

#[test]
#[should_panic(expected = "cannot find with empty needles")]
fn test_find_fwd_empty_needles_panic() {
    let needles: [u8; 0] = [];
    let haystack = [0x02, 0x03, 0x04];
    let at = 0;
    let result = find_fwd(&needles, &haystack, at);
}

