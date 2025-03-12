// Answer 0

#[test]
fn test_match_start_valid_start() {
    let haystack = "Hello, world!";
    let match_instance = Match::new(haystack, 0, 5);
    let _ = match_instance.start();
}

#[test]
fn test_match_start_non_empty_start() {
    let haystack = "Hello, world!";
    let match_instance = Match::new(haystack, 7, 12);
    let _ = match_instance.start();
}

#[test]
fn test_match_start_exact_end() {
    let haystack = "Hello, world!";
    let match_instance = Match::new(haystack, 13, 13);
    let _ = match_instance.start();
}

#[test]
fn test_match_start_middle() {
    let haystack = "Rust programming";
    let match_instance = Match::new(haystack, 5, 8);
    let _ = match_instance.start();
}

#[test]
fn test_match_start_boundary_condition() {
    let haystack = "Boundary condition.";
    let match_instance = Match::new(haystack, 0, 8);
    let _ = match_instance.start();
}

