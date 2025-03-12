// Answer 0

#[test]
fn test_index_into_object_valid_key() {
    struct TestIndex;

    impl Index for TestIndex {
        fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
            if let Value::Object(ref map) = *v {
                map.get("key")
            } else {
                None
            }
        }

        fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
            None
        }

        fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
            unimplemented!()
        }
    }

    let indexer = &TestIndex;
    let value = Value::Object(Map::new());
    let expected_value = Value::String("test".to_string());

    if let Value::Object(ref mut map) = value {
        map.insert("key".to_string(), expected_value);
    }

    let result = indexer.index_into(&value);
}

#[test]
fn test_index_into_array_valid_index() {
    struct TestIndex;

    impl Index for TestIndex {
        fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
            if let Value::Array(ref arr) = *v {
                arr.get(0)  // Testing with valid index 0
            } else {
                None
            }
        }

        fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
            None
        }

        fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
            unimplemented!()
        }
    }

    let indexer = &TestIndex;
    let value = Value::Array(vec![Value::String("first".to_string()), Value::String("second".to_string())]);
    
    let result = indexer.index_into(&value);
}

#[test]
fn test_index_into_object_not_found() {
    struct TestIndex;

    impl Index for TestIndex {
        fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
            if let Value::Object(ref map) = *v {
                map.get("nonexistent_key")  // Testing with a key that doesn't exist
            } else {
                None
            }
        }

        fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
            None
        }

        fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
            unimplemented!()
        }
    }

    let indexer = &TestIndex;
    let value = Value::Object(Map::new());
    
    let result = indexer.index_into(&value);
}

#[test]
fn test_index_into_array_out_of_bounds() {
    struct TestIndex;

    impl Index for TestIndex {
        fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
            if let Value::Array(ref arr) = *v {
                arr.get(10)  // Out-of-bounds index
            } else {
                None
            }
        }

        fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
            None
        }

        fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
            unimplemented!()
        }
    }

    let indexer = &TestIndex;
    let value = Value::Array(vec![Value::String("first".to_string())]);  // Only one element
    
    let result = indexer.index_into(&value);
}

