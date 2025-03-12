// Answer 0

#[test]
fn test_into_boxed_slice_empty() {
    struct TestIndexMap {
        entries: Vec<Bucket<i32, String>>,
    }
    
    impl TestIndexMap {
        pub fn into_entries(self) -> Vec<Bucket<i32, String>> {
            self.entries
        }

        pub fn into_boxed_slice(self) -> Box<Slice<i32, String>> {
            Slice::from_boxed(self.into_entries().into_boxed_slice())
        }
    }
    
    let map = TestIndexMap { entries: vec![] };
    let boxed_slice = map.into_boxed_slice();
}

#[test]
fn test_into_boxed_slice_single_entry() {
    struct TestIndexMap {
        entries: Vec<Bucket<i32, String>>,
    }
    
    impl TestIndexMap {
        pub fn into_entries(self) -> Vec<Bucket<i32, String>> {
            self.entries
        }

        pub fn into_boxed_slice(self) -> Box<Slice<i32, String>> {
            Slice::from_boxed(self.into_entries().into_boxed_slice())
        }
    }
    
    let map = TestIndexMap {
        entries: vec![Bucket { hash: 0.into(), key: 1, value: "one".to_string() }],
    };
    let boxed_slice = map.into_boxed_slice();
}

#[test]
fn test_into_boxed_slice_multiple_entries() {
    struct TestIndexMap {
        entries: Vec<Bucket<i32, String>>,
    }
    
    impl TestIndexMap {
        pub fn into_entries(self) -> Vec<Bucket<i32, String>> {
            self.entries
        }

        pub fn into_boxed_slice(self) -> Box<Slice<i32, String>> {
            Slice::from_boxed(self.into_entries().into_boxed_slice())
        }
    }
    
    let map = TestIndexMap {
        entries: vec![
            Bucket { hash: 0.into(), key: 1, value: "one".to_string() },
            Bucket { hash: 1.into(), key: 2, value: "two".to_string() },
        ],
    };
    let boxed_slice = map.into_boxed_slice();
}

