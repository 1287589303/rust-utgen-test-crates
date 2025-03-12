// Answer 0

#[test]
fn test_difference_unique_elements() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::SipHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::SipHasher::new()
        }
    }

    let mut set1: super::IndexSet<i32, TestHasher> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: TestHasher } };
    let mut set2: super::IndexSet<i32, TestHasher> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: TestHasher } };

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    set2.insert(4);
    set2.insert(5);
    set2.insert(6);

    let _diff = set1.difference(&set2);
}

#[test]
fn test_difference_overlapping_elements() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::SipHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::SipHasher::new()
        }
    }

    let mut set1: super::IndexSet<i32, TestHasher> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: TestHasher } };
    let mut set2: super::IndexSet<i32, TestHasher> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: TestHasher } };

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    set2.insert(2);
    set2.insert(3);
    set2.insert(4);

    let _diff = set1.difference(&set2);
}

#[test]
fn test_difference_identical_elements() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::SipHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::SipHasher::new()
        }
    }

    let mut set1: super::IndexSet<i32, TestHasher> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: TestHasher } };
    let mut set2: super::IndexSet<i32, TestHasher> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: TestHasher } };

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    set2.insert(1);
    set2.insert(2);
    set2.insert(3);
    
    let _diff = set1.difference(&set2);
}

#[test]
fn test_difference_empty_other_set() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::SipHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::SipHasher::new()
        }
    }

    let mut set1: super::IndexSet<i32, TestHasher> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: TestHasher } };
    let set2: super::IndexSet<i32, TestHasher> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: TestHasher } };

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);

    let _diff = set1.difference(&set2);
}

#[test]
fn test_difference_single_element_sets() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::SipHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::SipHasher::new()
        }
    }

    let mut set1: super::IndexSet<i32, TestHasher> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: TestHasher } };
    let mut set2: super::IndexSet<i32, TestHasher> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: TestHasher } };

    set1.insert(1);
    set2.insert(1);
    
    let _diff = set1.difference(&set2);
    
    set2.insert(2);
    let _diff_two = set1.difference(&set2);
    
    set1.insert(3);
    let _diff_three = set1.difference(&set2);
}

