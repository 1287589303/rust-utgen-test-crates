// Answer 0

#[test]
fn test_try_is_match_empty_haystack() {
    let re = BoundedBacktracker::new("foo[0-9]+bar").unwrap();
    let mut cache = re.create_cache();
    let input = Input::new(&b""[..]).span(0..0);
    re.try_is_match(&mut cache, input).unwrap();
}

#[test]
fn test_try_is_match_non_matching_haystack() {
    let re = BoundedBacktracker::new("foo[0-9]+bar").unwrap();
    let mut cache = re.create_cache();
    let input = Input::new(&b"foobar"[..]).span(0..6);
    let result = re.try_is_match(&mut cache, input);
    assert!(result.unwrap() == false);
}

#[test]
fn test_try_is_match_matching_haystack() {
    let re = BoundedBacktracker::new("foo[0-9]+bar").unwrap();
    let mut cache = re.create_cache();
    let input = Input::new(&b"foo12345bar"[..]).span(0..12);
    let result = re.try_is_match(&mut cache, input);
    assert!(result.unwrap() == true);
}

#[test]
fn test_try_is_match_anchored() {
    let re = BoundedBacktracker::new("^foo[0-9]+bar$").unwrap();
    let mut cache = re.create_cache();
    let input = Input::new(&b"foo123bar"[..]).anchored(Anchored::Yes).span(0..9);
    let result = re.try_is_match(&mut cache, input);
    assert!(result.unwrap() == false);
}

#[test]
fn test_try_is_match_earliest() {
    let re = BoundedBacktracker::new("foo[0-9]+bar").unwrap();
    let mut cache = re.create_cache();
    let input = Input::new(&b"foo12345bar"[..]).earliest(true).span(0..12);
    let result = re.try_is_match(&mut cache, input);
    assert!(result.unwrap() == true);
}

#[test]
fn test_try_is_match_utf8_mode() {
    let re = BoundedBacktracker::builder()
        .thompson(NFA::config().utf8(true))
        .build("a*").unwrap();
    let mut cache = re.create_cache();
    let input = Input::new("☃").span(0..1);
    let result = re.try_is_match(&mut cache, input);
    assert!(result.unwrap() == false);
}

#[test]
fn test_try_is_match_utf8_mode_disabled() {
    let re = BoundedBacktracker::builder()
        .thompson(NFA::config().utf8(false))
        .build("a*").unwrap();
    let mut cache = re.create_cache();
    let input = Input::new("☃").span(0..1);
    let result = re.try_is_match(&mut cache, input);
    assert!(result.unwrap() == true);
}

#[test]
fn test_try_is_match_span_out_of_bounds() {
    let re = BoundedBacktracker::new("foo[0-9]+bar").unwrap();
    let mut cache = re.create_cache();
    let input = Input::new(&b"foo12345bar"[..]).span(0..20);
    let result = re.try_is_match(&mut cache, input);
    assert!(result.is_err());
}

