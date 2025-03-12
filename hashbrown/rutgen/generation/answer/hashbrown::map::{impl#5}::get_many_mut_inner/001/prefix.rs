// Answer 0

#[test]
fn test_get_many_mut_inner_empty_array() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut hashmap: HashMap<u32, u32, DefaultHashBuilder, TestAllocator> = 
        HashMap { 
            hash_builder: DefaultHashBuilder::new(), 
            table: RawTable::new() 
        };

    let keys: [&u32; 0] = [];
    let _result = hashmap.get_many_mut_inner(&keys);
}

#[test]
fn test_get_many_mut_inner_single_element() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut hashmap: HashMap<u32, u32, DefaultHashBuilder, TestAllocator> = 
        HashMap { 
            hash_builder: DefaultHashBuilder::new(), 
            table: RawTable::new() 
        };

    let key: u32 = 1;
    let keys: [&u32; 1] = [&key];
    let _result = hashmap.get_many_mut_inner(&keys);
}

#[test]
fn test_get_many_mut_inner_multiple_elements() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut hashmap: HashMap<u32, u32, DefaultHashBuilder, TestAllocator> = 
        HashMap { 
            hash_builder: DefaultHashBuilder::new(), 
            table: RawTable::new() 
        };

    let key1: u32 = 1;
    let key2: u32 = 2;
    let keys: [&u32; 2] = [&key1, &key2];
    let _result = hashmap.get_many_mut_inner(&keys);
}

#[test]
fn test_get_many_mut_inner_max_elements() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut hashmap: HashMap<u32, u32, DefaultHashBuilder, TestAllocator> = 
        HashMap { 
            hash_builder: DefaultHashBuilder::new(), 
            table: RawTable::new() 
        };

    let keys: [&u32; 10] = [&1, &2, &3, &4, &5, &6, &7, &8, &9, &10];
    let _result = hashmap.get_many_mut_inner(&keys);
}

