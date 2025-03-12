// Answer 0

#[test]
fn test_read_captures_at_valid_case() {
    let pattern = Arc::from("a(b)");
    let meta = meta::Regex::new(pattern.as_ref()).unwrap();
    let regex = Regex { meta, pattern };

    let haystack: &[u8] = b"abc";
    let start: usize = 0;
    let mut locs = CaptureLocations(captures::Captures::new());

    let result = regex.read_captures_at(&mut locs, haystack, start);
}

#[test]
fn test_read_captures_at_boundary_start_zero() {
    let pattern = Arc::from("a(b)");
    let meta = meta::Regex::new(pattern.as_ref()).unwrap();
    let regex = Regex { meta, pattern };

    let haystack: &[u8] = b"abc";
    let start: usize = 0;
    let mut locs = CaptureLocations(captures::Captures::new());

    let result = regex.read_captures_at(&mut locs, haystack, start);
}

#[test]
fn test_read_captures_at_boundary_start_equal_length() {
    let pattern = Arc::from("a(b)");
    let meta = meta::Regex::new(pattern.as_ref()).unwrap();
    let regex = Regex { meta, pattern };

    let haystack: &[u8] = b"abc";
    let start: usize = haystack.len();
    let mut locs = CaptureLocations(captures::Captures::new());

    let result = regex.read_captures_at(&mut locs, haystack, start);
}

#[test]
fn test_read_captures_at_non_matching_case() {
    let pattern = Arc::from("d(e)");
    let meta = meta::Regex::new(pattern.as_ref()).unwrap();
    let regex = Regex { meta, pattern };

    let haystack: &[u8] = b"abc";
    let start: usize = 0;
    let mut locs = CaptureLocations(captures::Captures::new());

    let result = regex.read_captures_at(&mut locs, haystack, start);
}

#[test]
fn test_read_captures_at_large_haystack() {
    let pattern = Arc::from("c(d)");
    let meta = meta::Regex::new(pattern.as_ref()).unwrap();
    let regex = Regex { meta, pattern };

    let haystack: Vec<u8> = (0..4096).map(|x| x as u8).collect();
    let start: usize = 0;
    let mut locs = CaptureLocations(captures::Captures::new());

    let result = regex.read_captures_at(&mut locs, &haystack, start);
}

