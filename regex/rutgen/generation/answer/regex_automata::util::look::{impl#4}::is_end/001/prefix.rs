// Answer 0

#[test]
fn test_is_end_with_empty_haystack() {
    let haystack: &[u8] = &[];
    let matcher = LookMatcher::new();
    let at = 0;
    matcher.is_end(haystack, at);
}

#[test]
fn test_is_end_with_haystack_length_one() {
    let haystack: &[u8] = &[1];
    let matcher = LookMatcher::new();
    let at = 1;
    matcher.is_end(haystack, at);
}

#[test]
fn test_is_end_with_haystack_length_two() {
    let haystack: &[u8] = &[1, 2];
    let matcher = LookMatcher::new();
    let at = 2;
    matcher.is_end(haystack, at);
}

#[test]
fn test_is_end_with_haystack_length_three() {
    let haystack: &[u8] = &[1, 2, 3];
    let matcher = LookMatcher::new();
    let at = 3;
    matcher.is_end(haystack, at);
}

#[test]
fn test_is_end_with_haystack_length_four() {
    let haystack: &[u8] = &[1, 2, 3, 4];
    let matcher = LookMatcher::new();
    let at = 4;
    matcher.is_end(haystack, at);
}

#[test]
fn test_is_end_with_haystack_length_five() {
    let haystack: &[u8] = &[1, 2, 3, 4, 5];
    let matcher = LookMatcher::new();
    let at = 5;
    matcher.is_end(haystack, at);
}

#[test]
fn test_is_end_with_haystack_length_six() {
    let haystack: &[u8] = &[1, 2, 3, 4, 5, 6];
    let matcher = LookMatcher::new();
    let at = 6;
    matcher.is_end(haystack, at);
}

#[test]
fn test_is_end_with_haystack_length_seven() {
    let haystack: &[u8] = &[1, 2, 3, 4, 5, 6, 7];
    let matcher = LookMatcher::new();
    let at = 7;
    matcher.is_end(haystack, at);
}

#[test]
fn test_is_end_with_haystack_length_eight() {
    let haystack: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
    let matcher = LookMatcher::new();
    let at = 8;
    matcher.is_end(haystack, at);
}

#[test]
fn test_is_end_with_haystack_length_nine() {
    let haystack: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
    let matcher = LookMatcher::new();
    let at = 9;
    matcher.is_end(haystack, at);
}

#[test]
fn test_is_end_with_haystack_length_ten() {
    let haystack: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let matcher = LookMatcher::new();
    let at = 10;
    matcher.is_end(haystack, at);
}

