// Answer 0

#[test]
fn test_memchr_found_at_start() {
    let haystack: &[u8] = &[42, 1, 2, 3];
    let n1: u8 = 42;
    let _ = memchr(n1, haystack);
}

#[test]
fn test_memchr_found_in_middle() {
    let haystack: &[u8] = &[1, 2, 42, 3];
    let n1: u8 = 42;
    let _ = memchr(n1, haystack);
}

#[test]
fn test_memchr_found_at_end() {
    let haystack: &[u8] = &[1, 2, 3, 42];
    let n1: u8 = 42;
    let _ = memchr(n1, haystack);
}

#[test]
fn test_memchr_not_found() {
    let haystack: &[u8] = &[1, 2, 3];
    let n1: u8 = 4;
    let _ = memchr(n1, haystack);
}

#[test]
fn test_memchr_single_byte_haystack_found() {
    let haystack: &[u8] = &[42];
    let n1: u8 = 42;
    let _ = memchr(n1, haystack);
}

#[test]
fn test_memchr_single_byte_haystack_not_found() {
    let haystack: &[u8] = &[100];
    let n1: u8 = 42;
    let _ = memchr(n1, haystack);
}

#[test]
fn test_memchr_larger_haystack_found() {
    let haystack: &[u8] = &[0; 1024];
    let n1: u8 = 100;
    let mut extended_haystack = haystack.to_vec();
    extended_haystack[500] = n1;
    let _ = memchr(n1, &extended_haystack);
}

#[test]
fn test_memchr_larger_haystack_not_found() {
    let haystack: &[u8] = &[0; 1024];
    let n1: u8 = 100;
    let _ = memchr(n1, haystack);
}

#[test]
fn test_memchr_found_multiple_occurrences() {
    let haystack: &[u8] = &[1, 2, 42, 3, 42];
    let n1: u8 = 42;
    let _ = memchr(n1, haystack);
}

