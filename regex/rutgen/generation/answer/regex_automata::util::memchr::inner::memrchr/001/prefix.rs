// Answer 0

#[test]
fn test_memrchr_valid_case() {
    let n1: u8 = 100;
    let haystack: &[u8] = &[1, 2, 100, 3, 4];
    let _ = regex_automata::memrchr(n1, haystack);
}

#[test]
fn test_memrchr_last_element() {
    let n1: u8 = 50;
    let haystack: &[u8] = &[10, 20, 30, 40, 50];
    let _ = regex_automata::memrchr(n1, haystack);
}

#[test]
fn test_memrchr_multiple_occurrences() {
    let n1: u8 = 5;
    let haystack: &[u8] = &[5, 10, 5, 20, 30];
    let _ = regex_automata::memrchr(n1, haystack);
}

#[test]
fn test_memrchr_first_element() {
    let n1: u8 = 1;
    let haystack: &[u8] = &[1];
    let _ = regex_automata::memrchr(n1, haystack);
}

#[test]
fn test_memrchr_large_haystack() {
    let n1: u8 = 7;
    let haystack: Vec<u8> = vec![0; 1_000_000];
    haystack[999_999] = n1;
    let _ = regex_automata::memrchr(n1, &haystack);
}

#[test]
fn test_memrchr_no_occurrence() {
    let n1: u8 = 200;
    let haystack: &[u8] = &[10, 20, 30];
    let _ = regex_automata::memrchr(n1, haystack);
}

#[test]
fn test_memrchr_boundary_case_min() {
    let n1: u8 = 0;
    let haystack: &[u8] = &[255, 254, 0];
    let _ = regex_automata::memrchr(n1, haystack);
}

#[test]
fn test_memrchr_boundary_case_max() {
    let n1: u8 = 255;
    let haystack: &[u8] = &[0, 1, 255];
    let _ = regex_automata::memrchr(n1, haystack);
}

