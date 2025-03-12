// Answer 0

#[test]
fn test_try_insert_key_already_exists() {
    let mut map: HashMap<i32, &str> = HashMap::new();
    map.try_insert(1, "first").unwrap();
    match map.try_insert(1, "second") {
        Err(OccupiedError { entry, value }) => {
            let _ = entry.key();
            let _ = entry.get();
            let _ = value;
        }
        _ => panic!(),
    }
}

#[test]
fn test_try_insert_key_with_different_value() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.try_insert("key".to_string(), 10).unwrap();
    match map.try_insert("key".to_string(), 20) {
        Err(OccupiedError { entry, value }) => {
            let _ = entry.key();
            let _ = entry.get();
            let _ = value;
        }
        _ => panic!(),
    }
}

#[test]
fn test_try_insert_with_key_collision() {
    let mut map: HashMap<u64, f64> = HashMap::new();
    map.try_insert(42, 3.14).unwrap();
    match map.try_insert(42, 2.71) {
        Err(OccupiedError { entry, value }) => {
            let _ = entry.key();
            let _ = entry.get();
            let _ = value;
        }
        _ => panic!(),
    }
}

