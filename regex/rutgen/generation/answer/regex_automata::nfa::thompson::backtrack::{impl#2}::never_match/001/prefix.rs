// Answer 0

#[test]
fn test_never_match_empty_input() {
    let re = BoundedBacktracker::never_match().unwrap();
    let mut cache = re.create_cache();
    let _ = re.try_find_iter(&mut cache, "").next();
}

#[test]
fn test_never_match_non_empty_input() {
    let re = BoundedBacktracker::never_match().unwrap();
    let mut cache = re.create_cache();
    let _ = re.try_find_iter(&mut cache, "foo").next();
}

#[test]
fn test_never_match_repeated_input() {
    let re = BoundedBacktracker::never_match().unwrap();
    let mut cache = re.create_cache();
    let _ = re.try_find_iter(&mut cache, "bar").next();
}

#[test]
fn test_never_match_long_input() {
    let re = BoundedBacktracker::never_match().unwrap();
    let mut cache = re.create_cache();
    let _ = re.try_find_iter(&mut cache, "this is a long input string").next();
}

#[test]
fn test_never_match_special_characters() {
    let re = BoundedBacktracker::never_match().unwrap();
    let mut cache = re.create_cache();
    let _ = re.try_find_iter(&mut cache, "!@#$%^&*()").next();
}

