// Answer 0

#[test]
fn test_is_match_at_start_zero() {
    let set = RegexSet::new([r"\bbar\b", r"(?m)^bar$"]).unwrap();
    let hay = b"foobar";
    set.is_match_at(hay, 0);
}

#[test]
fn test_is_match_at_valid_start() {
    let set = RegexSet::new([r"bar"]).unwrap();
    let hay = b"foobar";
    set.is_match_at(hay, 3);
}

#[test]
#[should_panic]
fn test_is_match_at_start_out_of_bounds() {
    let set = RegexSet::new([r"\bbar\b"]).unwrap();
    let hay = b"foobar";
    set.is_match_at(hay, 7);
}

#[test]
fn test_is_match_at_full_length() {
    let set = RegexSet::new([r"^foo"]).unwrap();
    let hay = b"foobar";
    set.is_match_at(hay, 0);
}

#[test]
fn test_is_match_at_context_sensitive() {
    let set = RegexSet::new([r"\bbar\b", r"(?m)^bar$"]).unwrap();
    let hay = b"foobar";
    assert!(!set.is_match_at(hay, 3));
}

