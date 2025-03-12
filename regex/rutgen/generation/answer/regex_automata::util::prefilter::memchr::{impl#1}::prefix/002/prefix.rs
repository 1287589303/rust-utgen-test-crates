// Answer 0

#[test]
fn test_prefix_success_case() {
    let memchr = Memchr(42);
    let haystack = [42, 43, 44]; 
    let span = Span { start: 0, end: 1 }; 
    let result = memchr.prefix(&haystack, span);
}

#[test]
fn test_prefix_success_case_non_zero_start() {
    let memchr = Memchr(43);
    let haystack = [42, 43, 44]; 
    let span = Span { start: 1, end: 2 }; 
    let result = memchr.prefix(&haystack, span);
}

#[test]
fn test_prefix_success_case_at_end() {
    let memchr = Memchr(44);
    let haystack = [42, 43, 44]; 
    let span = Span { start: 2, end: 3 }; 
    let result = memchr.prefix(&haystack, span);
}

