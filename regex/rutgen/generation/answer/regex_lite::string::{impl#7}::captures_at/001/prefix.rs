// Answer 0

#[test]
fn test_captures_at_valid_start_0() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    let start = 0;
    let _ = re.captures_at(hay, start);
}

#[test]
fn test_captures_at_valid_start_1() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    let start = 1;
    let _ = re.captures_at(hay, start);
}

#[test]
fn test_captures_at_valid_start_2() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    let start = 2;
    let _ = re.captures_at(hay, start);
}

#[test]
fn test_captures_at_valid_start_len_minus_1() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    let start = hay.len() - 1;
    let _ = re.captures_at(hay, start);
}

#[test]
#[should_panic]
fn test_captures_at_invalid_start_len() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    let start = hay.len(); // This should panic
    let _ = re.captures_at(hay, start);
}

