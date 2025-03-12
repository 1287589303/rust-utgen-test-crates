// Answer 0

#[test]
fn test_search_captures_with_matching_pattern() {
    let re = Regex::new_many(&["[a-z0-9]{6}"]).unwrap();
    let mut cache = re.create_cache();
    let mut caps = re.create_captures();
    let haystack = "abc123";

    let input = Input::new(haystack);
    re.search_captures_with(&mut cache, &input, &mut caps);
}

#[test]
fn test_search_captures_with_non_matching_pattern() {
    let re = Regex::new(r"\d{3}").unwrap();
    let mut cache = re.create_cache();
    let mut caps = re.create_captures();
    let haystack = "foo";

    let input = Input::new(haystack);
    re.search_captures_with(&mut cache, &input, &mut caps);
}

#[test]
fn test_search_captures_with_edge_case_empty_haystack() {
    let re = Regex::new(r"\w+").unwrap();
    let mut cache = re.create_cache();
    let mut caps = re.create_captures();
    let haystack = "";

    let input = Input::new(haystack);
    re.search_captures_with(&mut cache, &input, &mut caps);
}

#[test]
fn test_search_captures_with_max_length_haystack() {
    let re = Regex::new(r"[a-z]{5,10}").unwrap();
    let mut cache = re.create_cache();
    let mut caps = re.create_captures();
    let haystack = "abcdefghij"; // 10 characters

    let input = Input::new(haystack);
    re.search_captures_with(&mut cache, &input, &mut caps);
}

#[test]
fn test_search_captures_with_anchored_pattern() {
    let re = Regex::new(r"^foo").unwrap();
    let mut cache = re.create_cache();
    let mut caps = re.create_captures();
    let haystack = "foobar";

    let input = Input::new(haystack).anchored(Anchored::Pattern(PatternID::must(0)));
    re.search_captures_with(&mut cache, &input, &mut caps);
}

#[test]
fn test_search_captures_with_non_matching_anchored_pattern() {
    let re = Regex::new(r"^bar").unwrap();
    let mut cache = re.create_cache();
    let mut caps = re.create_captures();
    let haystack = "foobar";

    let input = Input::new(haystack).anchored(Anchored::Pattern(PatternID::must(0)));
    re.search_captures_with(&mut cache, &input, &mut caps);
}

