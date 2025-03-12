// Answer 0

#[test]
fn test_empty_regex_set() {
    let set = RegexSet::empty();
    let matches: Vec<_> = set.matches(b"").into_iter().collect();
}

#[test]
fn test_single_match() {
    let set = RegexSet::new([r"\w+"]).unwrap();
    let matches: Vec<_> = set.matches(b"foo").into_iter().collect();
}

#[test]
fn test_single_no_match() {
    let set = RegexSet::new([r"\w+"]).unwrap();
    let matches: Vec<_> = set.matches(b"").into_iter().collect();
}

#[test]
fn test_multiple_matches() {
    let set = RegexSet::new([
        r"\w+",
        r"\d+",
        r"\pL+",
        r"foo",
        r"bar",
        r"barfoo",
        r"foobar",
    ])
    .unwrap();
    let matches: Vec<_> = set.matches(b"foobar").into_iter().collect();
}

#[test]
fn test_partial_matches() {
    let set = RegexSet::new([
        r"\w+",
        r"\d+",
        r"foo",
        r"bar",
    ])
    .unwrap();
    let matches: Vec<_> = set.matches(b"foo123bar").into_iter().collect();
}

#[test]
fn test_no_matches() {
    let set = RegexSet::new([r"invalid"]).unwrap();
    let matches: Vec<_> = set.matches(b"").into_iter().collect();
}

#[test]
fn test_special_characters() {
    let set = RegexSet::new([r"[!@#\$%\^&\*]+"]).unwrap();
    let matches: Vec<_> = set.matches(b"hello!@#").into_iter().collect();
}

#[test]
fn test_empty_haystack() {
    let set = RegexSet::new([r"\d+", r"\w+"]).unwrap();
    let matches: Vec<_> = set.matches(b"").into_iter().collect();
}

