// Answer 0

#[test]
fn test_shortest_match_found() {
    let re = Regex {
        meta: meta::Regex::new(r"a+").unwrap(),
        pattern: Arc::from("a+"),
    };
    let offset = re.shortest_match(b"aaaaa").unwrap();
}

#[test]
fn test_shortest_match_empty_haystack() {
    let re = Regex {
        meta: meta::Regex::new(r"a+").unwrap(),
        pattern: Arc::from("a+"),
    };
    let offset = re.shortest_match(b"").is_none();
}

#[test]
fn test_shortest_match_no_match() {
    let re = Regex {
        meta: meta::Regex::new(r"a+").unwrap(),
        pattern: Arc::from("a+"),
    };
    let offset = re.shortest_match(b"bbbbb").is_none();
}

#[test]
fn test_shortest_match_single_character() {
    let re = Regex {
        meta: meta::Regex::new(r"a").unwrap(),
        pattern: Arc::from("a"),
    };
    let offset = re.shortest_match(b"abc").unwrap();
}

#[test]
fn test_shortest_match_character_class() {
    let re = Regex {
        meta: meta::Regex::new(r"[a-z]").unwrap(),
        pattern: Arc::from("[a-z]"),
    };
    let offset = re.shortest_match(b"123a").unwrap();
}

#[test]
fn test_shortest_match_last_character() {
    let re = Regex {
        meta: meta::Regex::new(r"a").unwrap(),
        pattern: Arc::from("a"),
    };
    let offset = re.shortest_match(b"xyzabc").unwrap();
}

#[test]
fn test_shortest_match_starting_at_end() {
    let re = Regex {
        meta: meta::Regex::new(r"a").unwrap(),
        pattern: Arc::from("a"),
    };
    let offset = re.shortest_match_at(b"aaaaa", 4).unwrap();
}

#[test]
fn test_shortest_match_negative_start() {
    let re = Regex {
        meta: meta::Regex::new(r"a").unwrap(),
        pattern: Arc::from("a"),
    };
    let offset = re.shortest_match_at(b"aaaaa", usize::MAX); // testing for negative index since usize can't be negative
}

