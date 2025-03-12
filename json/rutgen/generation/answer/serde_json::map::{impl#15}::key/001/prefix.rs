// Answer 0

#[test]
fn test_entry_key_occupied() {
    let mut map = MapImpl::new();
    let key = String::from("occupied_key");
    let value = Value::String(String::from("some_value"));
    map.insert(key.clone(), value.clone());

    let entry = Entry::Occupied(OccupiedEntry {
        occupied: map.get_mut(&key).unwrap().into(),
    });

    let returned_key = entry.key();
}

#[test]
fn test_entry_key_occupied_non_empty() {
    let mut map = MapImpl::new();
    let key = String::from("non_empty_key");
    let value = Value::String(String::from("another_value"));
    map.insert(key.clone(), value.clone());

    let entry = Entry::Occupied(OccupiedEntry {
        occupied: map.get_mut(&key).unwrap().into(),
    });

    let returned_key = entry.key();
}

#[test]
fn test_entry_key_occupied_another_case() {
    let mut map = MapImpl::new();
    let key = String::from("another_occupied_key");
    let value = Value::String(String::from("value_for_another_key"));
    map.insert(key.clone(), value.clone());

    let entry = Entry::Occupied(OccupiedEntry {
        occupied: map.get_mut(&key).unwrap().into(),
    });

    let returned_key = entry.key();
}

