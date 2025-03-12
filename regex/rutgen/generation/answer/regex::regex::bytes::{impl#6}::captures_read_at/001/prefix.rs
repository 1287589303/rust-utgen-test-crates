// Answer 0

#[test]
fn test_captures_read_at_start_0() {
    let re = Regex::new(r"\bexample\b").unwrap();
    let hay = b"example";
    let mut locs = CaptureLocations(captures::Captures::new());
    let _ = re.captures_read_at(&mut locs, hay, 0);
}

#[test]
fn test_captures_read_at_start_length() {
    let re = Regex::new(r"\bexample\b").unwrap();
    let hay = b"example";
    let mut locs = CaptureLocations(captures::Captures::new());
    let _ = re.captures_read_at(&mut locs, hay, hay.len());
}

#[test]
fn test_captures_read_at_start_halfway() {
    let re = Regex::new(r"\bexam\b").unwrap();
    let hay = b"example";
    let mut locs = CaptureLocations(captures::Captures::new());
    let _ = re.captures_read_at(&mut locs, hay, 3);
}

#[test]
#[should_panic]
fn test_captures_read_at_start_out_of_bounds() {
    let re = Regex::new(r"\bexample\b").unwrap();
    let hay = b"example";
    let mut locs = CaptureLocations(captures::Captures::new());
    let _ = re.captures_read_at(&mut locs, hay, hay.len() + 1);
}

#[test]
fn test_captures_read_at_non_matching() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = b"eschew";
    let mut locs = CaptureLocations(captures::Captures::new());
    let _ = re.captures_read_at(&mut locs, hay, 2);
}

