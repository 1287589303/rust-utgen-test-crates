// Answer 0

#[test]
fn test_next_with_single_match() {
    let haystack = "hello world";
    let it = meta::FindMatches::new("world", haystack).collect::<Vec<_>>();
    let mut matches = Matches { haystack, it: &mut it.iter() };

    let result = matches.next();
    // Call the function under test to obtain the match
    let _ = result;
}

#[test]
fn test_next_with_multiple_matches() {
    let haystack = "banana bandana";
    let it = meta::FindMatches::new("an", haystack).collect::<Vec<_>>();
    let mut matches = Matches { haystack, it: &mut it.iter() };

    let first_result = matches.next();
    let second_result = matches.next();
    // Call the function under test to obtain the matches
    let _ = first_result;
    let _ = second_result;
}

#[test]
fn test_next_with_no_matches() {
    let haystack = "hello world";
    let it = meta::FindMatches::new("xyz", haystack).collect::<Vec<_>>();
    let mut matches = Matches { haystack, it: &mut it.iter() };

    let result = matches.next();
    // Call the function under test to obtain the match
    let _ = result;
}

#[test]
fn test_next_with_boundary_case() {
    let haystack = "abc";
    let it = meta::FindMatches::new("a", haystack).collect::<Vec<_>>();
    let mut matches = Matches { haystack, it: &mut it.iter() };

    let result = matches.next();
    // Call the function under test to obtain the match
    let _ = result;
}

