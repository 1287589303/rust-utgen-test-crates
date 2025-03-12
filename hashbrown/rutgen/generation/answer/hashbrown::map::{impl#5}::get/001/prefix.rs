// Answer 0

#[test]
fn test_get_existing_key() {
    struct MyKey(u32);
    struct MyValue<'a>(&'a str);
    
    let mut map = HashMap::new();
    map.insert(MyKey(1), MyValue("a"));
    let key = MyKey(1);
    let result = map.get(&key);
}

#[test]
fn test_get_another_existing_key() {
    struct MyKey(u32);
    struct MyValue<'a>(&'a str);
    
    let mut map = HashMap::new();
    map.insert(MyKey(2), MyValue("b"));
    let key = MyKey(2);
    let result = map.get(&key);
}

#[test]
fn test_get_multiple_existing_keys() {
    struct MyKey(u32);
    struct MyValue<'a>(&'a str);
    
    let mut map = HashMap::new();
    map.insert(MyKey(3), MyValue("c"));
    map.insert(MyKey(4), MyValue("d"));
    
    let key1 = MyKey(3);
    let result1 = map.get(&key1);
    
    let key2 = MyKey(4);
    let result2 = map.get(&key2);
}

