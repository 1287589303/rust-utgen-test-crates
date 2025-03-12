// Answer 0

#[test]
fn test_captures_at_with_empty_haystack() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let haystack = "";
    let start = haystack.len();
    re.captures_at(haystack, start);
}

#[test]
fn test_captures_at_with_haystack_and_start_out_of_bounds() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let haystack = "eschew";
    let start = haystack.len(); // This is `7`, which is equal to haystack.len()
    re.captures_at(haystack, start);
}

#[test]
fn test_captures_at_with_haystack_and_start_out_of_bounds_plus_one() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let haystack = "eschew";
    let start = haystack.len() + 1; // This is `8`, which is greater than haystack.len()
    re.captures_at(haystack, start);
}

