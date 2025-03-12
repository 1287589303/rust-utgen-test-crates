// Answer 0

#[test]
fn test_captures_iter_valid_pattern() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let hay = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939)";
    let captures = re.captures_iter(hay);
}

#[test]
fn test_captures_iter_no_matches() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let hay = "No matches here.";
    let captures = re.captures_iter(hay);
}

#[test]
fn test_captures_iter_empty_haystack() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let hay = "";
    let captures = re.captures_iter(hay);
}

#[test]
fn test_captures_iter_single_character() {
    let re = Regex::new(r"(\w)").unwrap();
    let hay = "A";
    let captures = re.captures_iter(hay);
}

#[test]
fn test_captures_iter_multiple_valid_captures() {
    let re = Regex::new(r"'(?<title>[^']+)'\s+\((?<year>[0-9]{4})\)").unwrap();
    let hay = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931)";
    let captures = re.captures_iter(hay);
}

#[test]
fn test_captures_iter_long_haystack() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let hay = "'A' (2000), 'B' (1999)".repeat(1000); // Create a long haystack
    let captures = re.captures_iter(&hay);
}

