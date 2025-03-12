// Answer 0

#[test]
fn test_prefix_success_case_full_span() {
    let haystack: &[u8] = b"hello, world!";
    let span = Span { start: 0, end: haystack.len() };
    let memmem = Memmem {
        finder: memchr::memmem::Finder::new(b"hello"),
    };
    let _ = memmem.prefix(haystack, span);
}

#[test]
fn test_prefix_success_case_partial_span() {
    let haystack: &[u8] = b"rust is fun!";
    let span = Span { start: 0, end: 14 }; // haystack length
    let memmem = Memmem {
        finder: memchr::memmem::Finder::new(b"rust"),
    };
    let _ = memmem.prefix(haystack, span);
}

#[test]
fn test_prefix_success_case_substring_in_middle() {
    let haystack: &[u8] = b"the quick brown fox";
    let span = Span { start: 0, end: haystack.len() };
    let memmem = Memmem {
        finder: memchr::memmem::Finder::new(b"quick"),
    };
    let _ = memmem.prefix(haystack, span);
}

#[test]
fn test_prefix_success_case_last_position() {
    let haystack: &[u8] = b"end with the needle";
    let span = Span { start: 0, end: haystack.len() };
    let memmem = Memmem {
        finder: memchr::memmem::Finder::new(b"needle"),
    };
    let _ = memmem.prefix(haystack, span);
}

