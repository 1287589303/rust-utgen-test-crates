// Answer 0

#[test]
fn test_find_with_perf_literal_multisubstring_enabled() {
    let teddy = Teddy {
        searcher: aho_corasick::packed::Searcher::new(&[b'a', b'b', b'c']),
        minimum_len: 1,
        _unused: (),
    };
    let haystack: &[u8] = b"abcdefgabcdefgh";
    let span = Span { start: 0, end: 16 };
    teddy.find(haystack, span);
}

#[test]
fn test_find_with_perf_literal_multisubstring_enabled_boundary_case() {
    let teddy = Teddy {
        searcher: aho_corasick::packed::Searcher::new(&[b'a', b'b', b'c']),
        minimum_len: 1,
        _unused: (),
    };
    let haystack: &[u8] = b"abcdefgh";
    let span = Span { start: 0, end: 8 };
    teddy.find(haystack, span);
}

#[test]
#[should_panic]
fn test_find_with_perf_literal_multisubstring_enabled_invalid_span() {
    let teddy = Teddy {
        searcher: aho_corasick::packed::Searcher::new(&[b'a', b'b', b'c']),
        minimum_len: 1,
        _unused: (),
    };
    let haystack: &[u8] = b"abcdefgh";
    let span = Span { start: 5, end: 3 }; // Invalid span where start >= end
    teddy.find(haystack, span);
}

#[test]
fn test_find_without_perf_literal_multisubstring_enabled() {
    let teddy = Teddy {
        _unused: (),
        searcher: aho_corasick::packed::Searcher::new(&[b'a', b'b', b'c']),
        minimum_len: 1,
    };
    let haystack: &[u8] = b"abcdefgabcdefgh";
    let span = Span { start: 0, end: 16 };
    teddy.find(haystack, span);
}

#[test]
fn test_find_without_perf_literal_multisubstring_enabled_boundary_case() {
    let teddy = Teddy {
        _unused: (),
        searcher: aho_corasick::packed::Searcher::new(&[b'a', b'b', b'c']),
        minimum_len: 1,
    };
    let haystack: &[u8] = b"abcdefgh";
    let span = Span { start: 0, end: 8 };
    teddy.find(haystack, span);
}

#[test]
#[should_panic]
fn test_find_without_perf_literal_multisubstring_enabled_invalid_span() {
    let teddy = Teddy {
        _unused: (),
        searcher: aho_corasick::packed::Searcher::new(&[b'a', b'b', b'c']),
        minimum_len: 1,
    };
    let haystack: &[u8] = b"abcdefgh";
    let span = Span { start: 5, end: 3 }; // Invalid span where start >= end
    teddy.find(haystack, span);
}

