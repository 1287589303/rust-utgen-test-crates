// Answer 0

#[test]
fn test_into_keys_with_string_keys() {
    let mut map = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    map.insert("c".to_string(), 3);
    
    let keys: IntoKeys<String, i32, Global> = map.into_keys();
}

#[test]
fn test_into_keys_with_integer_keys() {
    let mut map = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    
    let keys: IntoKeys<i32, &str, Global> = map.into_keys();
}

#[test]
fn test_into_keys_with_struct_keys() {
    #[derive(Hash, Eq, PartialEq)]
    struct Key {
        id: usize,
    }
    
    let mut map = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    map.insert(Key { id: 1 }, "first");
    map.insert(Key { id: 2 }, "second");
    map.insert(Key { id: 3 }, "third");
    
    let keys: IntoKeys<Key, &str, Global> = map.into_keys();
}

#[test]
fn test_into_keys_with_custom_allocator() {
    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            unimplemented!()
        }
    }

    let mut map = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), MyAllocator);
    map.insert("x", 10);
    map.insert("y", 20);
    
    let keys: IntoKeys<&str, i32, MyAllocator> = map.into_keys();
}

#[test]
fn test_into_keys_with_zero_capacity() {
    let mut map = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    map.insert("key1", 100);
    
    let keys: IntoKeys<&str, i32, Global> = map.into_keys();
}

