// Answer 0

#[test]
fn test_shortest_match_at_start_0() {
    let re = Regex { meta: meta::Regex::new(r"\bchew\b").unwrap(), pattern: Arc::from(r"\bchew\b") };
    let haystack = "eschew";
    let _ = re.shortest_match_at(haystack, 0);
}

#[test]
fn test_shortest_match_at_start_1() {
    let re = Regex { meta: meta::Regex::new(r"\bchew\b").unwrap(), pattern: Arc::from(r"\bchew\b") };
    let haystack = "eschew";
    let _ = re.shortest_match_at(haystack, 1);
}

#[test]
fn test_shortest_match_at_start_2() {
    let re = Regex { meta: meta::Regex::new(r"\bchew\b").unwrap(), pattern: Arc::from(r"\bchew\b") };
    let haystack = "eschew";
    let _ = re.shortest_match_at(haystack, 2);
}

#[test]
fn test_shortest_match_at_start_length() {
    let re = Regex { meta: meta::Regex::new(r"\bchew\b").unwrap(), pattern: Arc::from(r"\bchew\b") };
    let haystack = "eschew";
    let _ = re.shortest_match_at(haystack, haystack.len());
}

#[should_panic]
#[test]
fn test_shortest_match_at_start_length_plus_one() {
    let re = Regex { meta: meta::Regex::new(r"\bchew\b").unwrap(), pattern: Arc::from(r"\bchew\b") };
    let haystack = "eschew";
    let _ = re.shortest_match_at(haystack, haystack.len() + 1);
}

