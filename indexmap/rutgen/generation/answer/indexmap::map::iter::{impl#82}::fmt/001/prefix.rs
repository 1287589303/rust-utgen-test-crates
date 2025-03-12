// Answer 0

#[test]
fn test_splice_empty_map() {
    let mut map: IndexMap<u32, String, std::collections::hash_map::RandomState> = IndexMap::new();
    let empty_iter = vec![].into_iter();
    let mut splice = Splice {
        map: &mut map,
        tail: IndexMapCore { indices: Indices::new(), entries: Entries::new() },
        drain: vec![].into_iter(),
        replace_with: empty_iter,
    };
    let _ = splice.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_splice_single_element_map() {
    let mut map: IndexMap<u32, String, std::collections::hash_map::RandomState> = IndexMap::new();
    map.insert(1, "one".to_string());
    let single_iter = vec![(1, "one".to_string())].into_iter();
    let drain = vec![Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() }].into_iter();
    let mut splice = Splice {
        map: &mut map,
        tail: IndexMapCore { indices: Indices::new(), entries: Entries::new() },
        drain,
        replace_with: single_iter,
    };
    let _ = splice.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_splice_multiple_elements_map() {
    let mut map: IndexMap<u32, String, std::collections::hash_map::RandomState> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    let multi_iter = vec![(2, "two".to_string()), (3, "three".to_string())].into_iter();
    let drain = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() },
        Bucket { hash: HashValue::default(), key: 2, value: "two".to_string() },
    ].into_iter();
    let mut splice = Splice {
        map: &mut map,
        tail: IndexMapCore { indices: Indices::new(), entries: Entries::new() },
        drain,
        replace_with: multi_iter,
    };
    let _ = splice.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_splice_full_capacity_map() {
    let mut map: IndexMap<u32, String, std::collections::hash_map::RandomState> = IndexMap::new();
    for i in 0..1000 {
        map.insert(i, format!("value {}", i));
    }
    let full_iter = (0..1000).map(|i| (i, format!("new value {}", i)));
    let drain = vec![
        Bucket { hash: HashValue::default(), key: 500, value: "value 500".to_string() },
        Bucket { hash: HashValue::default(), key: 999, value: "value 999".to_string() },
    ].into_iter();
    let mut splice = Splice {
        map: &mut map,
        tail: IndexMapCore { indices: Indices::new(), entries: Entries::new() },
        drain,
        replace_with: full_iter,
    };
    let _ = splice.fmt(&mut fmt::Formatter::new());
}

