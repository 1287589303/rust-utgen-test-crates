// Answer 0

#[test]
fn test_is_match_word_negate_case_1() {
    let haystack: &[u8] = b"hello world"; 
    let look = Look::WordNegate;
    let at = 5; 
    look.is_match(haystack, at);
}

#[test]
fn test_is_match_word_negate_case_2() {
    let haystack: &[u8] = b"good_morning"; 
    let look = Look::WordNegate;
    let at = 4; 
    look.is_match(haystack, at);
}

#[test]
fn test_is_match_word_negate_case_3() {
    let haystack: &[u8] = b"__test__"; 
    let look = Look::WordNegate;
    let at = 3; 
    look.is_match(haystack, at);
}

#[test]
fn test_is_match_word_negate_case_4() {
    let haystack: &[u8] = b"word#test"; 
    let look = Look::WordNegate;
    let at = 4; 
    look.is_match(haystack, at);
}

#[test]
fn test_is_match_word_negate_case_5() {
    let haystack: &[u8] = b"check_this"; 
    let look = Look::WordNegate;
    let at = 5; 
    look.is_match(haystack, at);
}

