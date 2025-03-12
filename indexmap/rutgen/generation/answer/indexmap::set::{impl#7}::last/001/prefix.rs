// Answer 0

#[test]
fn test_last_non_empty_single_element() {
    struct TestIndexSet {
        items: Vec<i32>,
    }
    
    impl TestIndexSet {
        fn new(items: Vec<i32>) -> Self {
            TestIndexSet { items }
        }

        fn last(&self) -> Option<&i32> {
            self.items.last()
        }
    }

    let set = TestIndexSet::new(vec![42]);
    let _ = set.last();
}

#[test]
fn test_last_non_empty_multiple_elements() {
    struct TestIndexSet {
        items: Vec<i32>,
    }
    
    impl TestIndexSet {
        fn new(items: Vec<i32>) -> Self {
            TestIndexSet { items }
        }

        fn last(&self) -> Option<&i32> {
            self.items.last()
        }
    }

    let set = TestIndexSet::new(vec![1, 2, 3, 4, 5]);
    let _ = set.last();
}

#[test]
fn test_last_empty() {
    struct TestIndexSet {
        items: Vec<i32>,
    }
    
    impl TestIndexSet {
        fn new(items: Vec<i32>) -> Self {
            TestIndexSet { items }
        }

        fn last(&self) -> Option<&i32> {
            self.items.last()
        }
    }

    let set = TestIndexSet::new(vec![]);
    let _ = set.last();
}

#[test]
fn test_last_non_empty_with_different_types() {
    struct TestIndexSet {
        items: Vec<String>,
    }
    
    impl TestIndexSet {
        fn new(items: Vec<String>) -> Self {
            TestIndexSet { items }
        }

        fn last(&self) -> Option<&String> {
            self.items.last()
        }
    }

    let set = TestIndexSet::new(vec!["first".to_string(), "second".to_string(), "third".to_string()]);
    let _ = set.last();
}

