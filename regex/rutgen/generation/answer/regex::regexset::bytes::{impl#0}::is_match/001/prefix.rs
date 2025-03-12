// Answer 0

#[test]
fn test_is_match_with_ascii() {
    let set = RegexSet::new([r"\w+", r"\d+"]).unwrap();
    let haystack = b"hello";
    set.is_match(haystack);
}

#[test]
fn test_is_match_with_multibyte() {
    let set = RegexSet::new([r"\w+", r"\d+"]).unwrap();
    let haystack = "你好".as_bytes();
    set.is_match(haystack);
}

#[test]
fn test_is_match_empty_haystack() {
    let set = RegexSet::new([r"\w+", r"\d+"]).unwrap();
    let haystack: &[u8] = b"";
    set.is_match(haystack);
}

#[test]
fn test_is_match_with_special_characters() {
    let set = RegexSet::new([r"\w+", r"[!@#$%^&*()]"]).unwrap();
    let haystack = b"hello!";
    set.is_match(haystack);
}

#[test]
fn test_is_match_without_match() {
    let set = RegexSet::new([r"\d+"]).unwrap();
    let haystack = b"hello";
    set.is_match(haystack);
}

#[test]
fn test_is_match_boundary_case_length() {
    let set = RegexSet::new([r"\w+"]).unwrap();
    let haystack = b"";
    set.is_match(haystack);
    
    let haystack_full = b"A".repeat(1000);
    set.is_match(&haystack_full);
}

