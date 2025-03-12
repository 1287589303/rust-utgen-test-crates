// Answer 0

#[test]
fn test_always_match_empty_string() {
    let re = BoundedBacktracker::always_match().unwrap();
    let mut cache = re.create_cache();
    let _ = re.try_find_iter(&mut cache, "").next();
}

#[test]
fn test_always_match_non_empty_string() {
    let re = BoundedBacktracker::always_match().unwrap();
    let mut cache = re.create_cache();
    let _ = re.try_find_iter(&mut cache, "foo").next();
}

#[test]
fn test_always_match_large_string() {
    let re = BoundedBacktracker::always_match().unwrap();
    let mut cache = re.create_cache();
    let _ = re.try_find_iter(&mut cache, "a very long string used for testing purposes").next();
}

