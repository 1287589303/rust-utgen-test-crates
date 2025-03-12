// Answer 0

#[test]
fn test_retain_with_all_elements_retained() {
    struct TestMap {
        data: indexmap::IndexMap<i32, String>,
    }
    
    let mut map = TestMap {
        data: indexmap::IndexMap::new(),
    };
    map.data.insert(1, "One".to_string());
    map.data.insert(2, "Two".to_string());
    
    map.data.retain(|_, _| true);
    
    let _ = map.data.len();
}

#[test]
fn test_retain_with_no_elements_retained() {
    struct TestMap {
        data: indexmap::IndexMap<i32, String>,
    }
    
    let mut map = TestMap {
        data: indexmap::IndexMap::new(),
    };
    map.data.insert(1, "One".to_string());
    map.data.insert(2, "Two".to_string());
    
    map.data.retain(|_, _| false);
    
    let _ = map.data.len();
}

#[test]
fn test_retain_with_some_elements_retained() {
    struct TestMap {
        data: indexmap::IndexMap<i32, String>,
    }
    
    let mut map = TestMap {
        data: indexmap::IndexMap::new(),
    };
    map.data.insert(1, "One".to_string());
    map.data.insert(2, "Two".to_string());
    
    map.data.retain(|k, _| *k == 1);
    
    let _ = map.data.len();
}

#[test]
fn test_retain_with_mutable_values() {
    struct TestMap {
        data: indexmap::IndexMap<i32, i32>,
    }
    
    let mut map = TestMap {
        data: indexmap::IndexMap::new(),
    };
    map.data.insert(1, 10);
    map.data.insert(2, 20);
    
    map.data.retain(|_, v| {
        *v += 5;
        *v > 12
    });
    
    let _ = map.data.len();
}

