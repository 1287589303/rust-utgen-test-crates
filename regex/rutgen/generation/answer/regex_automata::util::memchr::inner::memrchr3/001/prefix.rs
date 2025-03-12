// Answer 0

#[test]
fn test_memrchr3_multiple_occurrences_at_end() {
    let haystack: &[u8] = &[1, 2, 3, 4, 5, 3, 2, 1];
    let result = memrchr3(1, 2, 3, haystack);
}

#[test]
fn test_memrchr3_single_occurrence_in_middle() {
    let haystack: &[u8] = &[5, 5, 1, 5, 5];
    let result = memrchr3(1, 2, 3, haystack);
}

#[test]
fn test_memrchr3_no_occurrences() {
    let haystack: &[u8] = &[4, 5, 6, 7, 8];
    let result = memrchr3(1, 2, 3, haystack);
}

#[test]
fn test_memrchr3_occurrences_at_start() {
    let haystack: &[u8] = &[1, 1, 2, 3, 4, 5];
    let result = memrchr3(1, 2, 3, haystack);
}

#[test]
fn test_memrchr3_boundary_case_zero() {
    let haystack: &[u8] = &[0, 255, 127];
    let result = memrchr3(0, 255, 127, haystack);
}

#[test]
fn test_memrchr3_large_haystack() {
    let haystack: &[u8] = &[5; 1000]; // array of 1000 elements, all 5
    let result = memrchr3(1, 2, 3, haystack);
}

#[test]
fn test_memrchr3_random_case() {
    let haystack: &[u8] = &[10, 20, 30, 40, 50, 60, 70, 80, 90];
    let result = memrchr3(30, 40, 50, haystack);
}

#[test]
fn test_memrchr3_multiple_occurrences() {
    let haystack: &[u8] = &[1, 2, 1, 2, 1];
    let result = memrchr3(1, 2, 3, haystack);
}

