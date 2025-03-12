// Answer 0

#[test]
fn test_captures_at_valid_start_zero() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "chewing";
    re.captures_at(hay, 0);
}

#[test]
fn test_captures_at_valid_start_mid() {
    let re = Regex::new(r"(\w+)ing").unwrap();
    let hay = "chewing";
    re.captures_at(hay, 0);
}

#[test]
fn test_captures_at_valid_start_non_zero() {
    let re = Regex::new(r"\w+").unwrap();
    let hay = "eschew";
    re.captures_at(hay, 1);
}

#[test]
fn test_captures_at_start_at_last_index() {
    let re = Regex::new(r"w$").unwrap();
    let hay = "eschew";
    re.captures_at(hay, 5);
}

#[test]
fn test_captures_at_near_end_index() {
    let re = Regex::new(r"che").unwrap();
    let hay = "eschew";
    re.captures_at(hay, 3);
}

