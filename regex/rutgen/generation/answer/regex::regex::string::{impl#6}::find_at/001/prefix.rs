// Answer 0

#[test]
fn test_find_at_valid_start_zero() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    let _ = re.find_at(hay, 0);
}

#[test]
fn test_find_at_valid_start_within_range() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    let _ = re.find_at(hay, 1);
}

#[test]
fn test_find_at_valid_start_equals_length() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    let _ = re.find_at(hay, hay.len());
}

#[should_panic]
#[test]
fn test_find_at_start_out_of_bounds() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    let _ = re.find_at(hay, hay.len() + 1);
}

