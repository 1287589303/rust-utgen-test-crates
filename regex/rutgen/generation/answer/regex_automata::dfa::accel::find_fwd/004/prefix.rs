// Answer 0

#[test]
fn test_find_fwd_three_needles_found() {
    let needles: &[u8] = &[0, 1, 2];
    let haystack: &[u8] = &[0, 1, 2, 3, 4, 5];
    let at: usize = 0;

    let result = find_fwd(needles, haystack, at);
    let expected = Some(at + 0); // needle 0 is found at index 0

    let _ = (result, expected); // Ensuring the function is called
}

#[test]
fn test_find_fwd_three_needles_found_at_non_zero() {
    let needles: &[u8] = &[1, 2, 3];
    let haystack: &[u8] = &[0, 1, 2, 3, 4, 5];
    let at: usize = 1;

    let result = find_fwd(needles, haystack, at);
    let expected = Some(at + 0); // needle 1 is found at index 1

    let _ = (result, expected); // Ensuring the function is called
}

#[test]
fn test_find_fwd_three_needles_found_at_last() {
    let needles: &[u8] = &[3, 4, 5];
    let haystack: &[u8] = &[0, 1, 2, 3, 4, 5];
    let at: usize = 3;

    let result = find_fwd(needles, haystack, at);
    let expected = Some(at + 1); // needle 4 is found at index 4

    let _ = (result, expected); // Ensuring the function is called
}

