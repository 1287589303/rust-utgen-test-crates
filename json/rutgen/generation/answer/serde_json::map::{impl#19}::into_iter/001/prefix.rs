// Answer 0

#[test]
fn test_into_iter_empty_map() {
    let mut map: Map<String, Value> = Map { map: MapImpl::new() };
    let iter: IterMut = map.into_iter();
}

#[test]
fn test_into_iter_single_element_map() {
    let mut map: Map<String, Value> = Map { map: MapImpl::new() };
    map.map.insert("key1".to_string(), Value::String("value1".to_string()));
    let iter: IterMut = map.into_iter();
}

#[test]
fn test_into_iter_multiple_elements_map() {
    let mut map: Map<String, Value> = Map { map: MapImpl::new() };
    for i in 0..10 {
        map.map.insert(format!("key{}", i), Value::Number(i.into()));
    }
    let iter: IterMut = map.into_iter();
}

#[cfg(feature = "preserve_order")]
#[test]
fn test_into_iter_ordered_map() {
    let mut map: Map<String, Value> = Map { map: MapImpl::new() };
    map.map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.map.insert("key2".to_string(), Value::Bool(true));
    let iter: IterMut = map.into_iter();
}

#[cfg(feature = "preserve_order")]
#[test]
fn test_into_iter_large_ordered_map() {
    let mut map: Map<String, Value> = Map { map: MapImpl::new() };
    for i in 0..100 {
        map.map.insert(format!("key{}", i), Value::Number(i.into()));
    }
    let iter: IterMut = map.into_iter();
}

