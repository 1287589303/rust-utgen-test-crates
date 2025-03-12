// Answer 0

#[test]
fn test_word_match_case_1() {
    let look = Look::Word;
    let haystack = &[b'a', b'b', b' ', b'1', b'2', b'_', b'a', b'c', b'@'];
    let at = 2; // ' ' (space) at index 2 is not a word character; 'b' before is a word character
    look.is_match(haystack, at);
}

#[test]
fn test_word_match_case_2() {
    let look = Look::Word;
    let haystack = &[b'a', b'b', b' ', b'1', b'2', b'_', b'a', b'c', b'@'];
    let at = 3; // '1' at index 3 is a word character; ' ' before is not
    look.is_match(haystack, at);
}

#[test]
fn test_word_match_case_3() {
    let look = Look::Word;
    let haystack = &[b'a', b'b', b' ', b'1', b'2', b'_', b'a', b'c', b'@'];
    let at = 4; // '2' at index 4 is a word character; '1' before is a word character
    look.is_match(haystack, at);
}

#[test]
fn test_word_match_case_4() {
    let look = Look::Word;
    let haystack = &[b'a', b'b', b' ', b'1', b'2', b'_', b'a', b'c', b'@'];
    let at = 5; // '_' at index 5 is a word character; '2' before is a word character
    look.is_match(haystack, at);
}

#[test]
fn test_word_match_case_5() {
    let look = Look::Word;
    let haystack = &[b'a', b'b', b' ', b'1', b'2', b'_', b'a', b'c', b'@'];
    let at = 6; // 'a' at index 6 is a word character; '_' before is a word character
    look.is_match(haystack, at);
}

