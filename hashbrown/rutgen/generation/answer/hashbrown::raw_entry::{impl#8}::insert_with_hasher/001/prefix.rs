// Answer 0

#[test]
fn test_insert_with_hasher_basic() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::<(&str, u32), DummyAllocator> {
        table: RawTableInner::default(),
        alloc: DummyAllocator,
        marker: PhantomData,
    };
    let key = "a";
    let value = 100;

    let hasher = |k: &&str| {
        let mut s = DefaultHasher::new();
        k.hash(&mut s);
        s.finish()
    };

    let hash = hasher(&key);
    let hash_builder = DummyHasher;

    let entry = RawVacantEntryMut {
        table: &mut table,
        hash_builder: &hash_builder,
    };

    entry.insert_with_hasher(hash, key, value, hasher);
}

#[test]
fn test_insert_with_hasher_empty_key() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::<(&str, u32), DummyAllocator> {
        table: RawTableInner::default(),
        alloc: DummyAllocator,
        marker: PhantomData,
    };
    let key = "";
    let value = 200;

    let hasher = |k: &&str| {
        let mut s = DefaultHasher::new();
        k.hash(&mut s);
        s.finish()
    };

    let hash = hasher(&key);
    let hash_builder = DummyHasher;

    let entry = RawVacantEntryMut {
        table: &mut table,
        hash_builder: &hash_builder,
    };

    entry.insert_with_hasher(hash, key, value, hasher);
}

#[test]
fn test_insert_with_hasher_special_character_key() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::<(&str, u32), DummyAllocator> {
        table: RawTableInner::default(),
        alloc: DummyAllocator,
        marker: PhantomData,
    };
    let key = "!@#$%^&*()";
    let value = 300;

    let hasher = |k: &&str| {
        let mut s = DefaultHasher::new();
        k.hash(&mut s);
        s.finish()
    };

    let hash = hasher(&key);
    let hash_builder = DummyHasher;

    let entry = RawVacantEntryMut {
        table: &mut table,
        hash_builder: &hash_builder,
    };

    entry.insert_with_hasher(hash, key, value, hasher);
}

#[test]
fn test_insert_with_hasher_large_value() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::<(u32, String), DummyAllocator> {
        table: RawTableInner::default(),
        alloc: DummyAllocator,
        marker: PhantomData,
    };
    let key = 42;
    let value = "A very long string to test large values".to_string();

    let hasher = |k: &u32| {
        let mut s = DefaultHasher::new();
        k.hash(&mut s);
        s.finish()
    };

    let hash = hasher(&key);
    let hash_builder = DummyHasher;

    let entry = RawVacantEntryMut {
        table: &mut table,
        hash_builder: &hash_builder,
    };

    entry.insert_with_hasher(hash, key, value, hasher);
}

