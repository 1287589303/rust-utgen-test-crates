// Answer 0

#[test]
fn test_find_fwd_with_two_needles_found() {
    let needles = &[0x41, 0x42]; // Valid 2-byte needle
    let haystack = [0x00, 0x01, 0x41, 0x42, 0x03]; // haystack containing needles
    let at = 0; // Starting search position
    let result = find_fwd(needles, &haystack, at);
}

#[test]
fn test_find_fwd_with_two_needles_found_at_edge() {
    let needles = &[0x41, 0x42]; // Valid 2-byte needle
    let haystack = [0x00, 0x01, 0x03, 0x41, 0x42]; // haystack where needles are found at the end
    let at = 2; // Starting search position
    let result = find_fwd(needles, &haystack, at);
}

#[test]
fn test_find_fwd_with_two_needles_not_found() {
    let needles = &[0x41, 0x42]; // Valid 2-byte needle
    let haystack = [0x00, 0x01, 0x03]; // haystack where needles are not found
    let at = 0; // Starting search position
    let result = find_fwd(needles, &haystack, at);
}

#[test]
#[should_panic(expected = "invalid needles length: 4")]
fn test_find_fwd_with_four_needles() {
    let needles = &[0x41, 0x42, 0x43, 0x44]; // Invalid 4-byte needle
    let haystack = [0x00, 0x01, 0x02]; // Any haystack
    let at = 0; // Starting search position
    let result = find_fwd(needles, &haystack, at);
}

#[test]
#[should_panic(expected = "cannot find with empty needles")]
fn test_find_fwd_with_empty_needles() {
    let needles = &[]; // Empty needles
    let haystack = [0x00, 0x01, 0x02]; // Any haystack
    let at = 0; // Starting search position
    let result = find_fwd(needles, &haystack, at);
}

