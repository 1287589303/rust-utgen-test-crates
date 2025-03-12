// Answer 0

#[test]
fn test_read_captures_at_empty_string() {
    let regex = Regex { meta: meta::Regex::new(r"a").unwrap(), pattern: Arc::from("a") };
    let mut locs = CaptureLocations(captures::Captures::new());
    let result = regex.read_captures_at(&mut locs, "", 0);
}

#[test]
fn test_read_captures_at_single_character_no_match() {
    let regex = Regex { meta: meta::Regex::new(r"a").unwrap(), pattern: Arc::from("a") };
    let mut locs = CaptureLocations(captures::Captures::new());
    let result = regex.read_captures_at(&mut locs, "b", 0);
}

#[test]
fn test_read_captures_at_single_character_match() {
    let regex = Regex { meta: meta::Regex::new(r"a").unwrap(), pattern: Arc::from("a") };
    let mut locs = CaptureLocations(captures::Captures::new());
    let result = regex.read_captures_at(&mut locs, "a", 0);
}

#[test]
fn test_read_captures_at_no_matches_multiple_characters() {
    let regex = Regex { meta: meta::Regex::new(r"a").unwrap(), pattern: Arc::from("a") };
    let mut locs = CaptureLocations(captures::Captures::new());
    let result = regex.read_captures_at(&mut locs, "bcdef", 0);
}

#[test]
fn test_read_captures_at_multiple_occurrences() {
    let regex = Regex { meta: meta::Regex::new(r"a").unwrap(), pattern: Arc::from("a") };
    let mut locs = CaptureLocations(captures::Captures::new());
    let result = regex.read_captures_at(&mut locs, "abcabc", 0);
}

#[test]
fn test_read_captures_at_special_characters() {
    let regex = Regex { meta: meta::Regex::new(r"[!@#]").unwrap(), pattern: Arc::from("[!@#]") };
    let mut locs = CaptureLocations(captures::Captures::new());
    let result = regex.read_captures_at(&mut locs, "!@#$%^&*", 0);
}

#[test]
fn test_read_captures_at_start_out_of_bounds() {
    let regex = Regex { meta: meta::Regex::new(r"a").unwrap(), pattern: Arc::from("a") };
    let mut locs = CaptureLocations(captures::Captures::new());
    let result = regex.read_captures_at(&mut locs, "abc", 4);
}

#[test]
fn test_read_captures_at_start_at_end_of_string() {
    let regex = Regex { meta: meta::Regex::new(r"a").unwrap(), pattern: Arc::from("a") };
    let mut locs = CaptureLocations(captures::Captures::new());
    let result = regex.read_captures_at(&mut locs, "abc", 3);
}

