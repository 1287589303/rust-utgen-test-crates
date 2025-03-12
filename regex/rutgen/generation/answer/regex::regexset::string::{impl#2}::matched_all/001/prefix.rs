// Answer 0

#[test]
fn test_matched_all_with_all_patterns_matching() {
    let set = RegexSet::new(&[
        r"^foo",
        r"[a-z]+\.com",
    ]).unwrap();
    let matches = set.matches("foo.example.com");
    matches.matched_all();
}

#[test]
fn test_matched_all_with_no_patterns_matching() {
    let set = RegexSet::new(&[
        r"^foo",
        r"[a-z]+\.com",
    ]).unwrap();
    let matches = set.matches("bar.example.net");
    matches.matched_all();
}

#[test]
fn test_matched_all_with_empty_string() {
    let set = RegexSet::new(&[
        r"^foo",
        r"[a-z]+\.com",
    ]).unwrap();
    let matches = set.matches("");
    matches.matched_all();
}

#[test]
fn test_matched_all_with_some_patterns_matching() {
    let set = RegexSet::new(&[
        r"^foo",
        r"[a-z]+\.com",
    ]).unwrap();
    let matches = set.matches("foo.anything");
    matches.matched_all();
}

#[test]
fn test_matched_all_with_single_character_string() {
    let set = RegexSet::new(&[
        r"^f",
        r"[a-z]+\.com",
    ]).unwrap();
    let matches = set.matches("f");
    matches.matched_all();
}

#[test]
fn test_matched_all_with_special_characters() {
    let set = RegexSet::new(&[
        r"^foo.*",
        r"[a-z]+\.com",
    ]).unwrap();
    let matches = set.matches("foo.example.com");
    matches.matched_all();
}

