// Answer 0

#[test]
fn test_regexset_empty() {
    let regex_set = RegexSet::empty();
    let result = regex_set.is_empty();
}

#[test]
fn test_regexset_non_empty() {
    let regex_set = RegexSet::new(["[a-z]", "[0-9]"]).unwrap();
    let result = regex_set.is_empty();
}

