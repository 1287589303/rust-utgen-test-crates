// Answer 0

#[test]
fn test_find_at_start_zero() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = b"eschew";
    let result = re.find_at(hay, 0);
}

#[test]
fn test_find_at_start_within_bounds() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = b"eschew";
    let result = re.find_at(hay, 2);
}

#[test]
fn test_find_at_start_at_last_byte() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = b"eschew";
    let result = re.find_at(hay, hay.len() - 1);
}

#[test]
fn test_find_at_start_at_haystack_length() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = b"eschew";
    let result = re.find_at(hay, hay.len());
}

#[should_panic]
#[test]
fn test_find_at_start_exceeding_haystack_length() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = b"eschew";
    let _result = re.find_at(hay, hay.len() + 1);
}

