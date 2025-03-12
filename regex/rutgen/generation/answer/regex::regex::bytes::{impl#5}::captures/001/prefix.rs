// Answer 0

#[test]
fn test_captures_named_group_valid_match() {
    let re = Regex::new(r"'(?<title>[^']+)'\s+\((?<year>\d{4})\)").unwrap();
    let hay = b"'Citizen Kane' (1941)";
    let caps = re.captures(hay).unwrap();
}

#[test]
fn test_captures_unnamed_group_valid_match() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let hay = b"'Inception' (2010)";
    let caps = re.captures(hay).unwrap();
}

#[test]
fn test_captures_no_match_empty_string() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let hay = b"";
    let caps = re.captures(hay);
}

#[test]
fn test_captures_no_match_invalid_format() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let hay = b"Not a movie format";
    let caps = re.captures(hay);
}

#[test]
fn test_captures_multiple_matches() {
    let re = Regex::new(r"'(?<title>[^']+)'\s+\((?<year>\d{4})\)").unwrap();
    let hay = b"'The Matrix' (1999), 'Avatar' (2009)";
    let caps_matrix = re.captures(&hay[..15]).unwrap();
    let caps_avatar = re.captures(&hay[20..]).unwrap();
}

#[test]
fn test_captures_exact_match() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let hay = b"'The Godfather' (1972)";
    let caps = re.captures(hay).unwrap();
}

#[test]
fn test_captures_no_match_partial_string() {
    let re = Regex::new(r"'(?<title>[^']+)'\s+\((?<year>\d{4})\)").unwrap();
    let hay = b"'Movie without year'";
    let caps = re.captures(hay);
}

