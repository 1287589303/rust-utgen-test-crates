// Answer 0

#[derive(Default)]
struct MapStruct {
    value: i32,
}

struct TestStruct {
    map: Option<Vec<MapStruct>>,
}

impl TestStruct {
    fn new(map: Option<Vec<MapStruct>>) -> Self {
        TestStruct { map }
    }

    fn lookup(&self, key: u32) -> usize {
        key as usize // Simple lookup for testing purposes
    }

    fn get(&self, key: u32) -> MapStruct {
        self.map
            .as_ref()
            .map_or_else(|| Default::default(), |map| map[self.lookup(key)].clone())
    }
}

#[test]
fn test_get_with_some_map() {
    let map_data = vec![MapStruct { value: 1 }, MapStruct { value: 2 }];
    let test_struct = TestStruct::new(Some(map_data));
    let result = test_struct.get(1);
    assert_eq!(result.value, 2);
}

#[test]
fn test_get_with_none_map() {
    let test_struct = TestStruct::new(None);
    let result = test_struct.get(0);
    assert_eq!(result.value, 0); // Default value
}

#[test]
fn test_get_with_empty_map() {
    let test_struct = TestStruct::new(Some(vec![]));
    let result = test_struct.get(0);
    assert_eq!(result.value, 0); // Default value for empty map
}

