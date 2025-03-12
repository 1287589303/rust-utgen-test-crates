// Answer 0

#[test]
fn test_replacen_with_limit_exact_matches() {
    let re = Regex::new(r"\d+").unwrap();
    let haystack = b"The year is 1973 and 1975.";
    let limit = 2;
    let replacement = b"NUM";

    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_limit_greater_than_matches() {
    let re = Regex::new(r"\d+").unwrap();
    let haystack = b"Year: 2020, Year: 2021, Year: 2022";
    let limit = 3;
    let replacement = b"NUMBER";

    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_single_match() {
    let re = Regex::new(r"[A-Za-z]+").unwrap();
    let haystack = b"Just one match here.";
    let limit = 1;
    let replacement = b"MATCH";

    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_no_initial_matches() {
    let re = Regex::new(r"[0-9]+").unwrap();
    let haystack = b"No numbers found here.";
    let limit = 1;
    let replacement = b"NO_MATCH";

    let result = re.replacen(haystack, limit, replacement);
}

