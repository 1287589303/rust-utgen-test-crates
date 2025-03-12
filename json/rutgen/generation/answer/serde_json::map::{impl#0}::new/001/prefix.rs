// Answer 0

#[test]
fn test_new_without_preserve_order() {
    let map: Map<String, Value> = Map::new();
}

#[cfg(feature = "preserve_order")]
#[test]
fn test_new_with_preserve_order() {
    let map: Map<String, Value> = Map::new();
}

