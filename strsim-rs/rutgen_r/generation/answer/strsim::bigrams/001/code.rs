// Answer 0

#[test]
fn test_bigrams_empty_string() {
    let result: Vec<(char, char)> = bigrams("").collect();
    assert_eq!(result, Vec::<(char, char)>::new());
}

#[test]
fn test_bigrams_single_char() {
    let result: Vec<(char, char)> = bigrams("a").collect();
    assert_eq!(result, Vec::<(char, char)>::new());
}

#[test]
fn test_bigrams_two_chars() {
    let result: Vec<(char, char)> = bigrams("ab").collect();
    assert_eq!(result, vec![('a', 'b')]);
}

#[test]
fn test_bigrams_three_chars() {
    let result: Vec<(char, char)> = bigrams("abc").collect();
    assert_eq!(result, vec![('a', 'b'), ('b', 'c')]);
}

#[test]
fn test_bigrams_with_spaces() {
    let result: Vec<(char, char)> = bigrams("a b").collect();
    assert_eq!(result, vec![('a', ' '), (' ', 'b')]);
}

#[test]
fn test_bigrams_repeated_chars() {
    let result: Vec<(char, char)> = bigrams("aaa").collect();
    assert_eq!(result, vec![('a', 'a'), ('a', 'a')]);
}

