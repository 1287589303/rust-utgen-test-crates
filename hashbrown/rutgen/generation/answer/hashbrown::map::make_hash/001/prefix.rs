// Answer 0

#[test]
fn test_make_hash_with_string() {
    use core::hash::{BuildHasher, Hasher};
    use std::collections::hash_map::DefaultHasher;

    struct HashBuilder(DefaultHasher);

    impl BuildHasher for HashBuilder {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let builder = HashBuilder(DefaultHasher::new());
    let value = "test string";
    let _ = make_hash(&builder, &value);
}

#[test]
fn test_make_hash_with_empty_string() {
    use core::hash::{BuildHasher, Hasher};
    use std::collections::hash_map::DefaultHasher;

    struct HashBuilder(DefaultHasher);

    impl BuildHasher for HashBuilder {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let builder = HashBuilder(DefaultHasher::new());
    let value = "";
    let _ = make_hash(&builder, &value);
}

#[test]
fn test_make_hash_with_integer() {
    use core::hash::{BuildHasher, Hasher};
    use std::collections::hash_map::DefaultHasher;

    struct HashBuilder(DefaultHasher);

    impl BuildHasher for HashBuilder {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let builder = HashBuilder(DefaultHasher::new());
    let value = 42;
    let _ = make_hash(&builder, &value);
}

#[test]
fn test_make_hash_with_struct() {
    use core::hash::{BuildHasher, Hasher, Hash};

    #[derive(Hash)]
    struct TestStruct {
        id: u32,
        name: String,
    }

    struct HashBuilder(std::collections::hash_map::DefaultHasher);

    impl BuildHasher for HashBuilder {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let builder = HashBuilder(std::collections::hash_map::DefaultHasher::new());
    let value = TestStruct { id: 1, name: String::from("test") };
    let _ = make_hash(&builder, &value);
}

#[test]
fn test_make_hash_with_vec() {
    use core::hash::{BuildHasher, Hasher};
    use std::collections::hash_map::DefaultHasher;

    struct HashBuilder(DefaultHasher);

    impl BuildHasher for HashBuilder {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let builder = HashBuilder(DefaultHasher::new());
    let value = vec![1, 2, 3, 4];
    let _ = make_hash(&builder, &value);
}

