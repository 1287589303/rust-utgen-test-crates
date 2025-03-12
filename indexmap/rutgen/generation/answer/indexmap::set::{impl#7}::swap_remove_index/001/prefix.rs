// Answer 0

#[test]
fn test_swap_remove_index_valid() {
    struct TestIndexSet {
        inner: super::IndexSet<i32, ()>,
    }

    let mut set = TestIndexSet {
        inner: super::IndexSet {
            map: super::IndexMap {
                core: super::IndexMapCore::new(),
                hash_builder: (),
            },
        },
    };

    set.inner.map.insert(1, ());
    set.inner.map.insert(2, ());
    set.inner.map.insert(3, ());

    let removed_value = set.inner.swap_remove_index(1);
}

#[test]
fn test_swap_remove_index_first() {
    struct TestIndexSet {
        inner: super::IndexSet<i32, ()>,
    }

    let mut set = TestIndexSet {
        inner: super::IndexSet {
            map: super::IndexMap {
                core: super::IndexMapCore::new(),
                hash_builder: (),
            },
        },
    };

    set.inner.map.insert(10, ());
    let removed_value = set.inner.swap_remove_index(0);
}

#[test]
fn test_swap_remove_index_last() {
    struct TestIndexSet {
        inner: super::IndexSet<i32, ()>,
    }

    let mut set = TestIndexSet {
        inner: super::IndexSet {
            map: super::IndexMap {
                core: super::IndexMapCore::new(),
                hash_builder: (),
            },
        },
    };

    set.inner.map.insert(20, ());
    set.inner.map.insert(30, ());

    let removed_value = set.inner.swap_remove_index(1);
}

#[test]
fn test_swap_remove_index_empty() {
    struct TestIndexSet {
        inner: super::IndexSet<i32, ()>,
    }

    let mut set = TestIndexSet {
        inner: super::IndexSet {
            map: super::IndexMap {
                core: super::IndexMapCore::new(),
                hash_builder: (),
            },
        },
    };

    let removed_value = set.inner.swap_remove_index(0);
}

