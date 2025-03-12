// Answer 0

#[test]
fn test_into_deserializer_with_hashmap() {
    use std::collections::HashMap;

    let map: HashMap<String, i32> = HashMap::new();
    let deserializer = map.into_deserializer();
}

#[test]
fn test_into_deserializer_with_vec() {
    let vec: Vec<i32> = Vec::new();
    let deserializer = vec.into_deserializer();
}

#[test]
fn test_into_deserializer_with_btreeset() {
    use std::collections::BTreeSet;

    let set: BTreeSet<i32> = BTreeSet::new();
    let deserializer = set.into_deserializer();
}

