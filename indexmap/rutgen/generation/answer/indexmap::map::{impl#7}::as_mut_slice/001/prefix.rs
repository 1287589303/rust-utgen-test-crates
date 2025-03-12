// Answer 0

#[test]
fn test_as_mut_slice_non_empty() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, String>] {
            &mut self.entries
        }

        fn as_mut_slice(&mut self) -> &mut Slice<i32, String> {
            Slice::from_mut_slice(self.as_entries_mut())
        }
    }

    let mut map = TestMap {
        entries: vec![Bucket { hash: 1, key: 1, value: "one".to_string() }, Bucket { hash: 2, key: 2, value: "two".to_string() }],
    };

    let slice = map.as_mut_slice();
}

#[test]
fn test_as_mut_slice_single_entry() {
    struct TestMap {
        entries: Vec<Bucket<char, i32>>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut [Bucket<char, i32>] {
            &mut self.entries
        }

        fn as_mut_slice(&mut self) -> &mut Slice<char, i32> {
            Slice::from_mut_slice(self.as_entries_mut())
        }
    }

    let mut map = TestMap {
        entries: vec![Bucket { hash: 1, key: 'a', value: 10 }],
    };

    let slice = map.as_mut_slice();
}

#[test]
fn test_as_mut_slice_multiple_entries() {
    struct TestMap {
        entries: Vec<Bucket<String, f64>>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut [Bucket<String, f64>] {
            &mut self.entries
        }

        fn as_mut_slice(&mut self) -> &mut Slice<String, f64> {
            Slice::from_mut_slice(self.as_entries_mut())
        }
    }

    let mut map = TestMap {
        entries: vec![
            Bucket { hash: 1, key: "first".to_string(), value: 1.0 },
            Bucket { hash: 2, key: "second".to_string(), value: 2.0 },
            Bucket { hash: 3, key: "third".to_string(), value: 3.0 },
        ],
    };

    let slice = map.as_mut_slice();
}

