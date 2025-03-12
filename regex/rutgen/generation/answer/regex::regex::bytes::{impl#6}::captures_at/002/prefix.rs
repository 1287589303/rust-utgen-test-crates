// Answer 0

#[test]
#[should_panic]
fn test_captures_at_with_start_greater_than_haystack_length() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = b"eschew";
    let _result = re.captures_at(hay, hay.len() + 1);
}

#[test]
fn test_captures_at_with_start_equals_haystack_length() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = b"eschew";
    let result = re.captures_at(hay, hay.len());
}

#[test]
fn test_captures_at_with_start_in_middle_no_match() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = b"eschew";
    let result = re.captures_at(hay, 2);
}

#[test]
fn test_captures_at_with_empty_haystack() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay: &[u8] = b"";
    let result = re.captures_at(hay, 0);
}

#[test]
fn test_captures_at_with_single_character_no_match() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = b"a";
    let result = re.captures_at(hay, 0);
}

