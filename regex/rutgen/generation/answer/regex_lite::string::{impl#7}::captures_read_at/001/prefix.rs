// Answer 0

#[test]
fn test_captures_read_at_valid() {
    let re = Regex::new(r"\w+").unwrap();
    let haystack = "hello world";
    let mut locs = re.captures_at(haystack, 0).unwrap();
    let start = 0;
    let _ = re.captures_read_at(&mut locs, haystack, start);
}

#[test]
fn test_captures_read_at_middle() {
    let re = Regex::new(r"\w+").unwrap();
    let haystack = "hello world";
    let mut locs = re.captures_at(haystack, 0).unwrap();
    let start = 6;
    let _ = re.captures_read_at(&mut locs, haystack, start);
}

#[test]
fn test_captures_read_at_boundaries() {
    let re = Regex::new(r"\w+").unwrap();
    let haystack = "hello world";
    
    let mut locs = re.captures_at(haystack, 0).unwrap();
    let start = haystack.len() - 5; // "world"
    let _ = re.captures_read_at(&mut locs, haystack, start);
}

#[test]
#[should_panic]
fn test_captures_read_at_out_of_bounds() {
    let re = Regex::new(r"\w+").unwrap();
    let haystack = "hello world";
    let mut locs = re.captures_at(haystack, 0).unwrap();
    let start = haystack.len(); // Out of bounds index
    let _ = re.captures_read_at(&mut locs, haystack, start);
}

