// Answer 0

#[test]
fn test_is_match_at_start_zero() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    re.is_match_at(hay, 0);
}

#[test]
fn test_is_match_at_start_one() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    re.is_match_at(hay, 1);
}

#[test]
fn test_is_match_at_start_two() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    re.is_match_at(hay, 2);
}

#[test]
fn test_is_match_at_start_boundary() {
    let re = Regex::new(r"\bch").unwrap();
    let hay = "chocolate";
    re.is_match_at(hay, hay.len());
}

#[should_panic]
#[test]
fn test_is_match_at_start_out_of_bounds() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    re.is_match_at(hay, hay.len() + 1);
}

