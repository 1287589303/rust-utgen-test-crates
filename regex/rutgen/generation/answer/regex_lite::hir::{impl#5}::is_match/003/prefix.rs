// Answer 0

#[test]
fn test_word_start_half_case_1() {
    let haystack: &[u8] = &[b'a', b'b', b'_', b'c', b'1', b'2', b'3', b' ', b' ', b'a'];
    let look = Look::WordStartHalf;
    let at = 1; 
    look.is_match(haystack, at);
}

#[test]
fn test_word_start_half_case_2() {
    let haystack: &[u8] = &[b'a', b'b', b'_', b'c', b'1', b'2', b'3', b' ', b' ', b'a'];
    let look = Look::WordStartHalf;
    let at = 2; 
    look.is_match(haystack, at);
}

#[test]
fn test_word_start_half_case_3() {
    let haystack: &[u8] = &[b'a', b'b', b'_', b'c', b'1', b'2', b'3', b' ', b' ', b'a'];
    let look = Look::WordStartHalf;
    let at = 3; 
    look.is_match(haystack, at);
}

#[test]
fn test_word_start_half_case_4() {
    let haystack: &[u8] = &[b'a', b'b', b'_', b'c', b'1', b'2', b'3', b' ', b' ', b'a'];
    let look = Look::WordStartHalf;
    let at = 4; 
    look.is_match(haystack, at);
}

#[test]
fn test_word_start_half_case_5() {
    let haystack: &[u8] = &[b'a', b'b', b'_', b'c', b'1', b'2', b'3', b' ', b' ', b'a'];
    let look = Look::WordStartHalf;
    let at = 5; 
    look.is_match(haystack, at);
}

#[test]
fn test_word_start_half_case_6() {
    let haystack: &[u8] = &[b'a', b'b', b'_', b'c', b'1', b'2', b'3', b' ', b' ', b'a'];
    let look = Look::WordStartHalf;
    let at = 6; 
    look.is_match(haystack, at);
}

#[test]
fn test_word_start_half_case_7() {
    let haystack: &[u8] = &[b'a', b'b', b'_', b'c', b'1', b'2', b'3', b' ', b' ', b'a'];
    let look = Look::WordStartHalf;
    let at = 7; 
    look.is_match(haystack, at);
}

#[test]
fn test_word_start_half_case_8() {
    let haystack: &[u8] = &[b'a', b'b', b'_', b'c', b'1', b'2', b'3', b' ', b' ', b'a'];
    let look = Look::WordStartHalf;
    let at = 8; 
    look.is_match(haystack, at);
}

