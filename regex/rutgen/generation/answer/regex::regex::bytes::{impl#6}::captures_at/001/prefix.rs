// Answer 0

#[test]
fn test_captures_at_start_zero() {
    let re = Regex::new(r"chew").unwrap();
    let hay = b"eschew";
    let result = re.captures_at(hay, 0);
}

#[test]
fn test_captures_at_start_within_bounds() {
    let re = Regex::new(r"chew").unwrap();
    let hay = b"eschew";
    let result = re.captures_at(hay, 2);
}

#[test]
fn test_captures_at_start_last_character() {
    let re = Regex::new(r"w$").unwrap();
    let hay = b"eschew";
    let result = re.captures_at(hay, 5);
}

#[test]
fn test_captures_at_start_middle() {
    let re = Regex::new(r"es").unwrap();
    let hay = b"eschew";
    let result = re.captures_at(hay, 0);
}

#[test]
#[should_panic]
fn test_captures_at_start_out_of_bounds() {
    let re = Regex::new(r"chew").unwrap();
    let hay = b"eschew";
    let _result = re.captures_at(hay, hay.len());
}

