// Answer 0

#[test]
fn test_next_valid_match() {
    let haystack: &[u8] = b"abcabcabc";
    let pattern: &[u8] = b"abc";
    let it = meta::FindMatches::new(pattern, haystack).unwrap();
    let mut matches = Matches { haystack, it };
    let result = matches.next();
}

#[test]
fn test_next_multiple_matches() {
    let haystack: &[u8] = b"ababababa";
    let pattern: &[u8] = b"aba";
    let it = meta::FindMatches::new(pattern, haystack).unwrap();
    let mut matches = Matches { haystack, it };

    for _ in 0..3 {
        let result = matches.next();
    }
}

#[test]
fn test_next_empty_match() {
    let haystack: &[u8] = b"";
    let pattern: &[u8] = b"";
    let it = meta::FindMatches::new(pattern, haystack).unwrap();
    let mut matches = Matches { haystack, it };
    let result = matches.next();
}

#[test]
fn test_next_no_matches() {
    let haystack: &[u8] = b"hello";
    let pattern: &[u8] = b"world";
    let it = meta::FindMatches::new(pattern, haystack).unwrap();
    let mut matches = Matches { haystack, it };
    let result = matches.next();
}

#[test]
fn test_next_boundary_conditions() {
    let haystack: &[u8] = b"a";
    let pattern: &[u8] = b"a";
    let it = meta::FindMatches::new(pattern, haystack).unwrap();
    let mut matches = Matches { haystack, it };
    let result = matches.next();

    let haystack: &[u8] = b"abc";
    let pattern: &[u8] = b"c";
    let it = meta::FindMatches::new(pattern, haystack).unwrap();
    let mut matches = Matches { haystack, it };
    let result = matches.next();
}

#[test]
fn test_next_large_haystack() {
    let haystack: Vec<u8> = vec![b'a'; 1000];
    let pattern: &[u8] = b'a';
    let it = meta::FindMatches::new(pattern, &haystack).unwrap();
    let mut matches = Matches { haystack: &haystack, it };
    let result = matches.next();
}

