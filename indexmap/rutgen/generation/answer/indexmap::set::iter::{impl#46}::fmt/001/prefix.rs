// Answer 0

#[test]
fn test_fmt_with_non_empty_union() {
    use core::hashers::DefaultHasher;
    use std::collections::HashSet;

    struct TestBuildHasher;

    impl BuildHasher for TestBuildHasher {
        type Hasher = DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let hasher = TestBuildHasher;
    let elements1: Vec<u32> = vec![1, 2, 3];
    let elements2: HashSet<u32> = vec![2, 3, 4].into_iter().collect();

    let index_set1 = IndexSet::from_iter(elements1.clone().into_iter());
    let index_set2 = IndexSet::from_iter(elements2.clone().into_iter());

    let union_instance = Union {
        iter: index_set1.iter().chain(index_set2.iter()),
    };

    let mut formatter = fmt::Formatter::new();
    let _ = union_instance.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_empty_union() {
    use core::hashers::DefaultHasher;
    use std::collections::HashSet;

    struct TestBuildHasher;

    impl BuildHasher for TestBuildHasher {
        type Hasher = DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let hasher = TestBuildHasher;
    let elements1: Vec<u32> = vec![];
    let elements2: HashSet<u32> = vec![1, 2, 3].into_iter().collect();

    let index_set1 = IndexSet::from_iter(elements1.clone().into_iter());
    let index_set2 = IndexSet::from_iter(elements2.clone().into_iter());

    let union_instance = Union {
        iter: index_set1.iter().chain(index_set2.iter()),
    };

    let mut formatter = fmt::Formatter::new();
    let _ = union_instance.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_boundaries() {
    use core::hashers::DefaultHasher;
    use std::collections::HashSet;

    struct TestBuildHasher;

    impl BuildHasher for TestBuildHasher {
        type Hasher = DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let hasher = TestBuildHasher;
    let elements1: Vec<u32> = vec![u32::MAX];
    let elements2: HashSet<u32> = vec![u32::MIN, u32::MAX - 1].into_iter().collect();

    let index_set1 = IndexSet::from_iter(elements1.clone().into_iter());
    let index_set2 = IndexSet::from_iter(elements2.clone().into_iter());

    let union_instance = Union {
        iter: index_set1.iter().chain(index_set2.iter()),
    };

    let mut formatter = fmt::Formatter::new();
    let _ = union_instance.fmt(&mut formatter);
}

