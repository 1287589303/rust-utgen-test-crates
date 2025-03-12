// Answer 0

#[test]
fn test_is_match_at_start_zero() {
    let set = RegexSet::new([r"foo", r"bar"]).unwrap();
    let haystack = "foobar";
    let result = set.is_match_at(haystack, 0);
}

#[test]
fn test_is_match_at_start_one() {
    let set = RegexSet::new([r"foo", r"bar"]).unwrap();
    let haystack = "foobar";
    let result = set.is_match_at(haystack, 1);
}

#[test]
fn test_is_match_at_start_middle() {
    let set = RegexSet::new([r"foo", r"bar"]).unwrap();
    let haystack = "foobar";
    let result = set.is_match_at(haystack, 3);
}

#[test]
fn test_is_match_at_end() {
    let set = RegexSet::new([r"foo", r"bar"]).unwrap();
    let haystack = "foobar";
    let result = set.is_match_at(haystack, haystack.len() - 1);
}

#[test]
fn test_is_match_at_empty_string() {
    let set = RegexSet::empty();
    let haystack = "";
    let result = set.is_match_at(haystack, 0);
}

#[should_panic]
fn test_is_match_at_panic_out_of_bounds() {
    let set = RegexSet::new([r"foo", r"bar"]).unwrap();
    let haystack = "foobar";
    let result = set.is_match_at(haystack, haystack.len() + 1);
}

#[test]
fn test_is_match_at_long_string() {
    let set = RegexSet::new([r"hello", r"world"]).unwrap();
    let haystack = "a".repeat(1000); // Use a long string of 1000 characters
    let result = set.is_match_at(haystack, 500);
}

#[test]
fn test_is_match_at_various_lengths() {
    let test_cases = vec![
        ("a", 0),
        ("abc", 1),
        ("abcd", 2),
        ("abcdef", 3),
        ("abcdefghij", 5),
    ];
    let set = RegexSet::new([r"a"]).unwrap();
    for (haystack, start) in test_cases {
        let result = set.is_match_at(haystack, start);
    }
}

