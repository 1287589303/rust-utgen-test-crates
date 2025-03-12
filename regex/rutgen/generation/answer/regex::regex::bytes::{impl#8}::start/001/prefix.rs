// Answer 0

#[test]
fn test_start_valid_range() {
    let haystack: &[u8] = b"example";
    let start = 0;
    let end = 7;
    let m = Match::new(haystack, start, end);
    let result = m.start();
}

#[test]
fn test_start_mid_range() {
    let haystack: &[u8] = b"example";
    let start = 3;
    let end = 6;
    let m = Match::new(haystack, start, end);
    let result = m.start();
}

#[test]
fn test_start_end_range() {
    let haystack: &[u8] = b"example";
    let start = 6;
    let end = 7;
    let m = Match::new(haystack, start, end);
    let result = m.start();
}

#[test]
#[should_panic]
fn test_start_out_of_bounds_low() {
    let haystack: &[u8] = b"example";
    let start = usize::MAX; // out of bounds
    let end = 7;
    let m = Match::new(haystack, start, end);
    let result = m.start();
}

#[test]
#[should_panic]
fn test_start_out_of_bounds_high() {
    let haystack: &[u8] = b"example";
    let start = 8; // out of bounds
    let end = 9; // would also be out of bounds
    let m = Match::new(haystack, start, end);
    let result = m.start();
}

