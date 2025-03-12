// Answer 0

#[test]
fn test_captures_iter_empty_input() {
    let re = PikeVM::new("foo(?P<numbers>[0-9]+)").unwrap();
    let mut cache = re.create_cache();
    let text = "";
    let _captures_matches = re.captures_iter(&mut cache, text);
}

#[test]
fn test_captures_iter_single_character() {
    let re = PikeVM::new("foo(?P<numbers>[0-9]+)").unwrap();
    let mut cache = re.create_cache();
    let text = "f";
    let _captures_matches = re.captures_iter(&mut cache, text);
}

#[test]
fn test_captures_iter_non_matching_input() {
    let re = PikeVM::new("foo(?P<numbers>[0-9]+)").unwrap();
    let mut cache = re.create_cache();
    let text = "bar";
    let _captures_matches = re.captures_iter(&mut cache, text);
}

#[test]
fn test_captures_iter_single_match() {
    let re = PikeVM::new("foo(?P<numbers>[0-9]+)").unwrap();
    let mut cache = re.create_cache();
    let text = "foo1";
    let _captures_matches = re.captures_iter(&mut cache, text);
}

#[test]
fn test_captures_iter_multiple_matches() {
    let re = PikeVM::new("foo(?P<numbers>[0-9]+)").unwrap();
    let mut cache = re.create_cache();
    let text = "foo1 foo12 foo123";
    let _captures_matches = re.captures_iter(&mut cache, text);
}

#[test]
fn test_captures_iter_overlapping_matches() {
    let re = PikeVM::new("(?P<overlap>foo)(?P<numbers>[0-9]+)").unwrap();
    let mut cache = re.create_cache();
    let text = "foo1 foo12 foo123foo2";
    let _captures_matches = re.captures_iter(&mut cache, text);
}

#[test]
fn test_captures_iter_multiple_and_empty() {
    let re = PikeVM::new("foo(?P<numbers>[0-9]*)").unwrap();
    let mut cache = re.create_cache();
    let text = "foo0 foo12 foo123 foo";
    let _captures_matches = re.captures_iter(&mut cache, text);
}

