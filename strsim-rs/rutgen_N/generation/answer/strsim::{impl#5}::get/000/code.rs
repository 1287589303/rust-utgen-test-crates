// Answer 0

#[derive(Default)]
struct ValueType {
    value: i32,
}

struct MyMap {
    lookup: Vec<usize>,
    values: Vec<ValueType>,
}

impl MyMap {
    fn new(lookup: Vec<usize>, values: Vec<ValueType>) -> Self {
        MyMap { lookup, values }
    }

    fn get(&self, key: u32) -> ValueType {
        self.map
            .as_ref()
            .map_or_else(|| Default::default(), |map| map[self.lookup(key)].value)
    }
}

#[test]
fn test_get_with_empty_map() {
    let my_map = MyMap::new(vec![], vec![]);
    let result = my_map.get(0);
    assert_eq!(result.value, 0); // Assuming the default value is 0
}

#[test]
fn test_get_with_valid_key() {
    let values = vec![ValueType { value: 10 }, ValueType { value: 20 }];
    let lookup = vec![0, 1];
    let my_map = MyMap::new(lookup, values);
    let result = my_map.get(0);
    assert_eq!(result.value, 10);
}

#[test]
fn test_get_with_invalid_key() {
    let values = vec![ValueType { value: 30 }];
    let lookup = vec![0];
    let my_map = MyMap::new(lookup, values);
    let result = my_map.get(1); // Key out of bounds
    assert_eq!(result.value, 0); // Assuming the default value is 0
}

