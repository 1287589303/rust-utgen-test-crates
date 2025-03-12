// Answer 0

#[test]
fn test_is_fast_with_perf_literal_multisubstring_enabled() {
    let ac = AhoCorasick {
        #[cfg(not(feature = "perf-literal-multisubstring"))]
        _unused: (),
        #[cfg(feature = "perf-literal-multisubstring")]
        ac: aho_corasick::AhoCorasick::new(),
    };
    let result = ac.is_fast();
}

#[test]
#[should_panic]
fn test_is_fast_without_perf_literal_multisubstring() {
    let ac = AhoCorasick {
        #[cfg(not(feature = "perf-literal-multisubstring"))]
        _unused: (),
        #[cfg(feature = "perf-literal-multisubstring")] // This block should not be executed.
        ac: aho_corasick::AhoCorasick::new(),
    };
    let result = ac.is_fast();
}

