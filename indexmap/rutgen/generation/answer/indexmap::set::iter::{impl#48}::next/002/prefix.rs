// Answer 0

#[test]
fn test_next_with_valid_iterator() {
    use std::hash::{BuildHasher, Hasher};
    use std::collections::HashMap;

    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut data = vec![1, 2, 3];
    let iter = data.iter().cloned();
    let hasher = SimpleHasher;

    let mut splice = Splice {
        iter: crate::map::Splice::new(iter, hasher),
    };

    let result = splice.next();
}

#[test]
fn test_next_with_empty_iterator() {
    use std::hash::{BuildHasher, Hasher};
    use std::collections::HashMap;

    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let data: Vec<i32> = vec![];
    let iter = data.iter().cloned();
    let hasher = SimpleHasher;

    let mut splice = Splice {
        iter: crate::map::Splice::new(iter, hasher),
    };

    let result = splice.next();
}

