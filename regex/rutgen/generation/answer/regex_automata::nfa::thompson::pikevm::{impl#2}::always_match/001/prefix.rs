// Answer 0

#[test]
fn test_always_match_empty_string() {
    let pikevm = PikeVM::always_match().unwrap();
    let mut cache = pikevm.create_cache();
    let expected = Match::must(0, 0..0);
    let _ = pikevm.find_iter(&mut cache, "").next();
    let _ = pikevm.find_iter(&mut cache, "foo").next();
}

#[test]
fn test_always_match_non_empty_string() {
    let pikevm = PikeVM::always_match().unwrap();
    let mut cache = pikevm.create_cache();
    let expected = Match::must(0, 0..0);
    let _ = pikevm.find_iter(&mut cache, "some non-empty string").next();
    let _ = pikevm.find_iter(&mut cache, "another string").next();
}

