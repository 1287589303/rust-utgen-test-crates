// Answer 0

#[test]
fn test_len_with_non_empty_regex_set() {
    use regex::RegexSet;
    
    let set = RegexSet::new([
        r"[a-z]+@[a-z]+\.(com|org|net)",
        r"[a-z]+\.(com|org|net)",
    ]).unwrap();
    
    let matches = set.matches("example.com");
    let length = matches.len();
}

#[test]
fn test_len_with_empty_string() {
    use regex::RegexSet;
    
    let set = RegexSet::new([
        r"[a-z]+@[a-z]+\.(com|org|net)",
        r"[a-z]+\.(com|org|net)",
    ]).unwrap();
    
    let matches = set.matches("");
    let length = matches.len();
}

#[test]
fn test_len_with_no_matches() {
    use regex::RegexSet;
    
    let set = RegexSet::new([
        r"[a-z]+@[a-z]+\.(com|org|net)",
        r"[a-z]+\.(com|org|net)",
    ]).unwrap();
    
    let matches = set.matches("1234");
    let length = matches.len();
}

#[test]
fn test_len_with_single_pattern() {
    use regex::RegexSet;
    
    let set = RegexSet::new([
        r"[0-9]+",
    ]).unwrap();
    
    let matches = set.matches("1234");
    let length = matches.len();
}

#[test]
fn test_len_with_maximum_size_regex_set() {
    use regex::RegexSet;
    
    let patterns: Vec<&str> = (0..1000).map(|i| format!(r"pattern{}", i)).collect();
    let set = RegexSet::new(&patterns).unwrap();
    
    let matches = set.matches("example");
    let length = matches.len();
}

