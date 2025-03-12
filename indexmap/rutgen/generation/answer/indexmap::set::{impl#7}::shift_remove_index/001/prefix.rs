// Answer 0

#[test]
fn test_shift_remove_index_valid_index() {
    struct TestMutableValues {
        values: Vec<i32>,
    }

    impl private::Sealed for TestMutableValues {}
    impl MutableValues for TestMutableValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.values.get_mut(index)
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {
        }
    }

    let mut index_set = IndexSet {
        map: IndexMap {
            core: IndexMapCore {
                entries: TestMutableValues { values: vec![1, 2, 3] },
            },
            hash_builder: RandomState::new(),
        },
    };
    
    let result = index_set.shift_remove_index(1);
}

#[test]
fn test_shift_remove_index_empty() {
    struct TestMutableValues {
        values: Vec<i32>,
    }

    impl private::Sealed for TestMutableValues {}
    impl MutableValues for TestMutableValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            None
        }

        fn get_index_mut2(&mut self, _index: usize) -> Option<&mut Self::Value> {
            None
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {
        }
    }

    let mut index_set: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore {
                entries: TestMutableValues { values: vec![] },
            },
            hash_builder: RandomState::new(),
        },
    };
    
    let result = index_set.shift_remove_index(0);
}

#[test]
fn test_shift_remove_index_boundary_start() {
    struct TestMutableValues {
        values: Vec<i32>,
    }

    impl private::Sealed for TestMutableValues {}
    impl MutableValues for TestMutableValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.values.get_mut(index)
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {
        }
    }

    let mut index_set = IndexSet {
        map: IndexMap {
            core: IndexMapCore {
                entries: TestMutableValues { values: vec![5, 10, 15] },
            },
            hash_builder: RandomState::new(),
        },
    };
    
    let result = index_set.shift_remove_index(0);
}

#[test]
fn test_shift_remove_index_boundary_end() {
    struct TestMutableValues {
        values: Vec<i32>,
    }

    impl private::Sealed for TestMutableValues {}
    impl MutableValues for TestMutableValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.values.get_mut(index)
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {
        }
    }

    let mut index_set = IndexSet {
        map: IndexMap {
            core: IndexMapCore {
                entries: TestMutableValues { values: vec![100, 200, 300] },
            },
            hash_builder: RandomState::new(),
        },
    };
    
    let result = index_set.shift_remove_index(2);
}

