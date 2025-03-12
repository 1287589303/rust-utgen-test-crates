// Answer 0

#[test]
fn test_search_captures_basic_match() {
    let regex = Regex::new(r"[a-z][a-z0-9]{5}").unwrap();
    let mut captures = Captures {
        group_info: GroupInfo::default(),
        pid: None,
        slots: vec![Some(NonMaxUsize(NonZeroUsize::new(0).unwrap())); 2],
    };
    let haystack: &[u8] = b"abc123";
    let input = Input {
        haystack,
        span: Span::new(0..6),
        anchored: Anchored::No,
        earliest: false,
    };
    regex.search_captures(&input, &mut captures);
}

#[test]
fn test_search_captures_no_match() {
    let regex = Regex::new(r"[0-9]{3}").unwrap();
    let mut captures = Captures {
        group_info: GroupInfo::default(),
        pid: None,
        slots: vec![None; 2],
    };
    let haystack: &[u8] = b"abc";
    let input = Input {
        haystack,
        span: Span::new(0..3),
        anchored: Anchored::No,
        earliest: false,
    };
    regex.search_captures(&input, &mut captures);
}

#[test]
fn test_search_captures_anchored_match() {
    let regex = Regex::new(r"foo[a-z]{3}").unwrap();
    let mut captures = Captures {
        group_info: GroupInfo::default(),
        pid: None,
        slots: vec![Some(NonMaxUsize(NonZeroUsize::new(0).unwrap())); 2],
    };
    let haystack: &[u8] = b"foobar";
    let input = Input {
        haystack,
        span: Span::new(0..6),
        anchored: Anchored::Pattern(PatternID::must(0)),
        earliest: false,
    };
    regex.search_captures(&input, &mut captures);
}

#[test]
fn test_search_captures_empty_haystack() {
    let regex = Regex::new(r"[a-z]{3}").unwrap();
    let mut captures = Captures {
        group_info: GroupInfo::default(),
        pid: None,
        slots: vec![None; 2],
    };
    let haystack: &[u8] = b"";
    let input = Input {
        haystack,
        span: Span::new(0..0),
        anchored: Anchored::No,
        earliest: false,
    };
    regex.search_captures(&input, &mut captures);
}

#[test]
fn test_search_captures_multiple_patterns() {
    let regex = Regex::new_many(&["foo", "bar"]).unwrap();
    let mut captures = Captures {
        group_info: GroupInfo::default(),
        pid: None,
        slots: vec![Some(NonMaxUsize(NonZeroUsize::new(0).unwrap())); 2],
    };
    let haystack: &[u8] = b"foobar";
    let input = Input {
        haystack,
        span: Span::new(0..6),
        anchored: Anchored::No,
        earliest: false,
    };
    regex.search_captures(&input, &mut captures);
}

