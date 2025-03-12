// Answer 0

#[test]
fn test_new_prefilter_leftmost_first() {
    let needles: &[&[u8]] = &[b"needle1", b"needle2"];
    let prefilter = Prefilter::new(MatchKind::LeftmostFirst, needles).expect("a prefilter");
    let haystack: &[u8] = b"This is a test with needle1 and needle2";
    let span = Span::from(0..haystack.len());
    let _result = prefilter.find(haystack, span);
}

#[test]
fn test_new_prefilter_all() {
    let needles: &[&[u8]] = &[b"needleA"];
    let prefilter = Prefilter::new(MatchKind::All, needles).expect("a prefilter");
    let haystack: &[u8] = b"needleA is a needleA test";
    let span = Span::from(0..haystack.len());
    let _result = prefilter.find(haystack, span);
}

#[test]
fn test_new_prefilter_empty_needles() {
    let needles: &[&[u8]] = &[];
    let prefilter = Prefilter::new(MatchKind::LeftmostFirst, needles);
    assert!(prefilter.is_none());
}

#[test]
fn test_new_prefilter_single_byte_needle() {
    let needles: &[&[u8]] = &[b"a"];
    let prefilter = Prefilter::new(MatchKind::LeftmostFirst, needles).expect("a prefilter");
    let haystack: &[u8] = b"abcdef";
    let span = Span::from(0..haystack.len());
    let _result = prefilter.find(haystack, span);
}

#[test]
fn test_new_prefilter_large_needle() {
    let long_needle = vec![b'a'; 1024]; // 1024 bytes of 'a'
    let needles: &[&[u8]] = &[&long_needle];
    let prefilter = Prefilter::new(MatchKind::All, needles).expect("a prefilter");
    let haystack: &[u8] = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let span = Span::from(0..haystack.len());
    let _result = prefilter.find(haystack, span);
}

#[test]
fn test_new_prefilter_needles_with_empty_strings() {
    let needles: &[&[u8]] = &[b"needle", b""];
    let prefilter = Prefilter::new(MatchKind::All, needles);
    assert!(prefilter.is_none());
}

