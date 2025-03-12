// Answer 0

#[test]
fn test_as_parts_mut_valid_cache() {
    let regex = Regex::new("test").unwrap();
    let mut cache = Cache::new(&regex);
    let (forward_cache, reverse_cache) = cache.as_parts_mut();
}

#[test]
fn test_as_parts_mut_empty_cache() {
    let regex = Regex::new("").unwrap();
    let mut cache = Cache::new(&regex);
    let (forward_cache, reverse_cache) = cache.as_parts_mut();
}

#[test]
fn test_as_parts_mut_large_cache() {
    let regex = Regex::new(".*").unwrap();
    let mut cache = Cache::new(&regex);
    let (forward_cache, reverse_cache) = cache.as_parts_mut();
}

