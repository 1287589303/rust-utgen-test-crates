// Answer 0

#[test]
fn test_sort_by_identity() {
    struct TestSet {
        values: Vec<i32>,
    }

    impl MutableValues for TestSet {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _: &Q) -> Option<(usize, &mut Self::Value)>
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

    let mut set = TestSet {
        values: vec![3, 1, 2],
    };

    set.sort_by(|a, b| a.cmp(b));
}

#[test]
fn test_sort_by_descending() {
    struct TestSet {
        values: Vec<i32>,
    }

    impl MutableValues for TestSet {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _: &Q) -> Option<(usize, &mut Self::Value)>
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

    let mut set = TestSet {
        values: vec![1, 3, 2],
    };

    set.sort_by(|a, b| b.cmp(a));
}

#[test]
fn test_sort_by_custom_order() {
    struct TestSet {
        values: Vec<i32>,
    }

    impl MutableValues for TestSet {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _: &Q) -> Option<(usize, &mut Self::Value)>
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

    let mut set = TestSet {
        values: vec![5, 2, 8, 1],
    };

    set.sort_by(|a, b| (a % 2).cmp(&(b % 2)).then(a.cmp(b)));
}

#[test]
fn test_sort_by_single_element() {
    struct TestSet {
        values: Vec<i32>,
    }

    impl MutableValues for TestSet {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _: &Q) -> Option<(usize, &mut Self::Value)>
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

    let mut set = TestSet {
        values: vec![42],
    };

    set.sort_by(|a, b| a.cmp(b));
}

#[test]
fn test_sort_by_already_sorted() {
    struct TestSet {
        values: Vec<i32>,
    }

    impl MutableValues for TestSet {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _: &Q) -> Option<(usize, &mut Self::Value)>
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

    let mut set = TestSet {
        values: vec![1, 2, 3],
    };

    set.sort_by(|a, b| a.cmp(b));
}

#[test]
fn test_sort_by_reverse_sorted() {
    struct TestSet {
        values: Vec<i32>,
    }

    impl MutableValues for TestSet {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _: &Q) -> Option<(usize, &mut Self::Value)>
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

    let mut set = TestSet {
        values: vec![3, 2, 1],
    };

    set.sort_by(|a, b| a.cmp(b));
}

