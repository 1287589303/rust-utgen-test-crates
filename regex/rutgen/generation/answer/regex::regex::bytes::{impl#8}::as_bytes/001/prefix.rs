// Answer 0

#[test]
fn test_as_bytes_non_empty() {
    let haystack: &[u8] = b"Hello, world!";
    let m = Match::new(haystack, 0, 5);
    let result = m.as_bytes();
}

#[test]
fn test_as_bytes_entire_haystack() {
    let haystack: &[u8] = b"Hello, world!";
    let m = Match::new(haystack, 0, haystack.len());
    let result = m.as_bytes();
}

#[test]
fn test_as_bytes_partial_match() {
    let haystack: &[u8] = b"Hello, world!";
    let m = Match::new(haystack, 7, 12);
    let result = m.as_bytes();
}

#[test]
fn test_as_bytes_empty_match() {
    let haystack: &[u8] = b"Hello, world!";
    let m = Match::new(haystack, 5, 5);
    let result = m.as_bytes();
}

#[test]
#[should_panic]
fn test_as_bytes_start_equals_length() {
    let haystack: &[u8] = b"Hello, world!";
    let m = Match::new(haystack, haystack.len(), haystack.len());
    let result = m.as_bytes();
}

#[test]
fn test_as_bytes_boundary_conditions() {
    let haystack: &[u8] = b"Test";
    let start = 0;
    let end = 4;
    let m = Match::new(haystack, start, end);
    let result = m.as_bytes();

    let haystack_large: Vec<u8> = (0..1000).map(|i| i as u8).collect();
    let start_large = 0;
    let end_large = 1000;
    let m_large = Match::new(&haystack_large, start_large, end_large);
    let result_large = m_large.as_bytes();
}

