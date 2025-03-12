// Answer 0

#[test]
fn test_count_with_single_match() {
    let haystack = "abcde";
    let pattern = "bc";
    let it = meta::FindMatches::new(pattern, haystack);
    let matches = Matches { haystack, it };
    let count = matches.count();
}

#[test]
fn test_count_with_no_matches() {
    let haystack = "abcdef";
    let pattern = "gh";
    let it = meta::FindMatches::new(pattern, haystack);
    let matches = Matches { haystack, it };
    let count = matches.count();
}

#[test]
fn test_count_with_multiple_matches() {
    let haystack = "abcabcabc";
    let pattern = "abc";
    let it = meta::FindMatches::new(pattern, haystack);
    let matches = Matches { haystack, it };
    let count = matches.count();
}

#[test]
fn test_count_with_long_haystack() {
    let haystack = "a".repeat(10000);
    let pattern = "a";
    let it = meta::FindMatches::new(pattern, &haystack);
    let matches = Matches { haystack: &haystack, it };
    let count = matches.count();
}

#[test]
fn test_count_with_pattern_longer_than_haystack() {
    let haystack = "abc";
    let pattern = "abcdef";
    let it = meta::FindMatches::new(pattern, haystack);
    let matches = Matches { haystack, it };
    let count = matches.count();
}

#[test]
fn test_count_with_empty_pattern() {
    let haystack = "abc";
    let pattern = "";
    let it = meta::FindMatches::new(pattern, haystack);
    let matches = Matches { haystack, it };
    let count = matches.count();
}

