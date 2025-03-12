// Answer 0

#[test]
fn test_and_modify_vacant_entry() {
    struct TestMap {
        inner: MapImpl<String, Value>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                inner: MapImpl::new(),
            }
        }

        fn entry(&mut self, key: &str) -> Entry {
            if self.inner.contains_key(key) {
                Entry::Occupied(OccupiedEntry {
                    occupied: self.inner.get_mut(key).unwrap(),
                })
            } else {
                let vacant = VacantEntry {
                    vacant: self.inner.entry(String::from(key)).or_insert(Value::Null),
                };
                Entry::Vacant(vacant)
            }
        }
    }

    let mut map = TestMap::new();
    let entry = map.entry("vacant_key");

    let _ = entry.and_modify(|e| *e = Value::Bool(true));
} 

#[test]
fn test_and_modify_vacant_entry_with_object() {
    struct TestMap {
        inner: MapImpl<String, Value>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                inner: MapImpl::new(),
            }
        }

        fn entry(&mut self, key: &str) -> Entry {
            if self.inner.contains_key(key) {
                Entry::Occupied(OccupiedEntry {
                    occupied: self.inner.get_mut(key).unwrap(),
                })
            } else {
                let vacant = VacantEntry {
                    vacant: self.inner.entry(String::from(key)).or_insert(Value::Object(MapImpl::new())),
                };
                Entry::Vacant(vacant)
            }
        }
    }

    let mut map = TestMap::new();
    let entry = map.entry("vacant_object_entry");

    let _ = entry.and_modify(|e| *e = Value::Object(MapImpl::new()));
} 

#[test]
fn test_and_modify_vacant_entry_with_array() {
    struct TestMap {
        inner: MapImpl<String, Value>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                inner: MapImpl::new(),
            }
        }

        fn entry(&mut self, key: &str) -> Entry {
            if self.inner.contains_key(key) {
                Entry::Occupied(OccupiedEntry {
                    occupied: self.inner.get_mut(key).unwrap(),
                })
            } else {
                let vacant = VacantEntry {
                    vacant: self.inner.entry(String::from(key)).or_insert(Value::Array(vec![])),
                };
                Entry::Vacant(vacant)
            }
        }
    }

    let mut map = TestMap::new();
    let entry = map.entry("vacant_array_entry");

    let _ = entry.and_modify(|e| *e = Value::Array(vec![Value::Number(1.into()), Value::Number(2.into())]));
}

