// Answer 0

#[test]
fn test_get_index_mut_valid_index() {
    struct TestMap {
        data: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                data: vec![
                    Bucket { hash: HashValue::default(), key: 1, value: 10 }, 
                    Bucket { hash: HashValue::default(), key: 2, value: 20 }
                ],
            }
        }

        fn as_entries_mut(&mut self) -> &mut Vec<Bucket<i32, i32>> {
            &mut self.data
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let mut map = TestMap::new();
    map.get_index_mut(0);
    map.get_index_mut(1);
}

#[test]
fn test_get_index_mut_out_of_bounds() {
    struct TestMap {
        data: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                data: vec![
                    Bucket { hash: HashValue::default(), key: 1, value: 10 }, 
                    Bucket { hash: HashValue::default(), key: 2, value: 20 }
                ],
            }
        }

        fn as_entries_mut(&mut self) -> &mut Vec<Bucket<i32, i32>> {
            &mut self.data
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let mut map = TestMap::new();
    let _ = map.get_index_mut(2); // Out of bounds case
    let _ = map.get_index_mut(usize::MAX); // Also out of bounds
}

