// Answer 0

#[test]
fn test_is_match_valid_pattern_matching() {
    let set = RegexSet::new(["\\w+", "\\d+"]).unwrap();
    let result1 = set.is_match("foo");
    let result2 = set.is_match("123");
    let result3 = set.is_match("test123");
}

#[test]
fn test_is_match_valid_pattern_non_matching() {
    let set = RegexSet::new(["\\w+", "\\d+"]).unwrap();
    let result1 = set.is_match("☃");
    let result2 = set.is_match("!@#$%");
}

#[test]
fn test_is_match_edge_cases() {
    let set = RegexSet::new(["\\w+", "\\d+"]).unwrap();
    let result_empty = set.is_match("");
    let result_single_char_alpha = set.is_match("a");
    let result_single_char_numeric = set.is_match("1");
    let result_single_char_symbol = set.is_match("$");
}

#[test]
fn test_is_match_unicode_characters() {
    let set = RegexSet::new(["\\w+", "\\d+"]).unwrap();
    let result_unicode = set.is_match("测试");
    let result_mixed = set.is_match("abc123测试");
}

#[test]
fn test_is_match_long_haystack() {
    let set = RegexSet::new(["\\w+", "\\d+"]).unwrap();
    let long_matching = set.is_match("a".repeat(1000));
    let long_non_matching = set.is_match("☃".repeat(1000));
}

#[test]
fn test_is_match_boundaries() {
    let set = RegexSet::new(["^abc", "xyz$"]).unwrap();
    let result_start_match = set.is_match("abc is the start");
    let result_end_match = set.is_match("this is xyz");
    let result_no_match_start = set.is_match("not matching abc here");
    let result_no_match_end = set.is_match("xyz is not here");
}

