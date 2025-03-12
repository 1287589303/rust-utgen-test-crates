// Answer 0

#[test]
fn test_iter_with_multiple_matches() {
    let set = SetMatches(PatternSet::new(vec![
        r"[0-9]",
        r"[a-z]",
        r"[A-Z]",
        r"\p{Greek}",
    ]).unwrap());
    let hay = "βa1";
    let matches: Vec<_> = set.iter().collect();
}

#[test]
fn test_iter_with_no_matches() {
    let set = SetMatches(PatternSet::new(vec![
        r"[0-9]",
        r"[a-z]",
        r"[A-Z]",
    ]).unwrap());
    let hay = "!@#$%";
    let matches: Vec<_> = set.iter().collect();
}

#[test]
fn test_iter_with_empty_string() {
    let set = SetMatches(PatternSet::new(vec![
        r"[0-9]",
        r"[a-z]",
        r"[A-Z]",
    ]).unwrap());
    let hay = "";
    let matches: Vec<_> = set.iter().collect();
}

#[test]
fn test_iter_with_special_character_patterns() {
    let set = SetMatches(PatternSet::new(vec![
        r"[!@#$%^&*()]",
        r"[0-9]",
        r"[a-z]",
    ]).unwrap());
    let hay = "!";
    let matches: Vec<_> = set.iter().collect();
}

#[test]
fn test_iter_with_boundary_cases() {
    let set = SetMatches(PatternSet::new(vec![
        r"[0-9]",
        r"[a-z]",
        r"[A-Z]",
        r"\p{Greek}",
        r"\s",
    ]).unwrap());
    let hay = "123abcXYZ ";
    let matches: Vec<_> = set.iter().collect();
}

