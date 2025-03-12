// Answer 0

#[test]
fn test_captures_read_with_valid_input() {
    let re = Regex::new(r"^([a-z]+)=(\S*)$").unwrap();
    let mut locs = re.capture_locations();
    let haystack = "id=foo123";
    let result = re.captures_read(&mut locs, haystack);
    let _ = result; // Calling the function to test with valid input
}

#[test]
fn test_captures_read_with_empty_haystack() {
    let re = Regex::new(r"^([a-z]+)=(\S*)$").unwrap();
    let mut locs = re.capture_locations();
    let haystack = "";
    let result = re.captures_read(&mut locs, haystack);
    let _ = result; // Calling the function to test with an empty haystack
}

#[test]
fn test_captures_read_with_non_matching_haystack() {
    let re = Regex::new(r"^([a-z]+)=(\S*)$").unwrap();
    let mut locs = re.capture_locations();
    let haystack = "foo123"; // No '=' present, does not match
    let result = re.captures_read(&mut locs, haystack);
    let _ = result; // Calling the function to test with a non-matching haystack
}

#[test]
fn test_captures_read_with_partially_matching_haystack() {
    let re = Regex::new(r"^([a-z]+)=(\S*)$").unwrap();
    let mut locs = re.capture_locations();
    let haystack = "id=123 =foo"; // Should partially match, testing boundary conditions
    let result = re.captures_read(&mut locs, haystack);
    let _ = result; // Calling the function to test with a partially matching haystack
}

