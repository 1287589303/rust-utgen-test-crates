// Answer 0

#[test]
fn test_get_range_mut_valid_range() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, i32>] {
            &mut self.entries
        }

        fn get_range_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Slice<i32, i32>> {
            let entries = self.as_entries_mut();
            let range = try_simplify_range(range, entries.len())?;
            entries.get_mut(range).map(Slice::from_mut_slice)
        }
    }

    let mut map = TestMap {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: 10 },
            Bucket { hash: HashValue::default(), key: 2, value: 20 },
            Bucket { hash: HashValue::default(), key: 3, value: 30 },
        ],
    };

    let _result = map.get_range_mut(0..2);
}

#[test]
fn test_get_range_mut_exclusive_end() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, i32>] {
            &mut self.entries
        }

        fn get_range_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Slice<i32, i32>> {
            let entries = self.as_entries_mut();
            let range = try_simplify_range(range, entries.len())?;
            entries.get_mut(range).map(Slice::from_mut_slice)
        }
    }

    let mut map = TestMap {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: 10 },
            Bucket { hash: HashValue::default(), key: 2, value: 20 },
            Bucket { hash: HashValue::default(), key: 3, value: 30 },
        ],
    };

    let _result = map.get_range_mut(1..3);
}

#[test]
fn test_get_range_mut_empty() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, i32>] {
            &mut self.entries
        }

        fn get_range_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Slice<i32, i32>> {
            let entries = self.as_entries_mut();
            let range = try_simplify_range(range, entries.len())?;
            entries.get_mut(range).map(Slice::from_mut_slice)
        }
    }

    let mut map = TestMap { entries: Vec::new() };

    let _result = map.get_range_mut(0..0);
}

#[test]
fn test_get_range_mut_full_range() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, i32>] {
            &mut self.entries
        }

        fn get_range_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Slice<i32, i32>> {
            let entries = self.as_entries_mut();
            let range = try_simplify_range(range, entries.len())?;
            entries.get_mut(range).map(Slice::from_mut_slice)
        }
    }

    let mut map = TestMap {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: 10 },
            Bucket { hash: HashValue::default(), key: 2, value: 20 },
            Bucket { hash: HashValue::default(), key: 3, value: 30 },
        ],
    };

    let _result = map.get_range_mut(0..3);
}

#[test]
fn test_get_range_mut_start_equals_end() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, i32>] {
            &mut self.entries
        }

        fn get_range_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Slice<i32, i32>> {
            let entries = self.as_entries_mut();
            let range = try_simplify_range(range, entries.len())?;
            entries.get_mut(range).map(Slice::from_mut_slice)
        }
    }

    let mut map = TestMap {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: 10 },
            Bucket { hash: HashValue::default(), key: 2, value: 20 },
            Bucket { hash: HashValue::default(), key: 3, value: 30 },
        ],
    };

    let _result = map.get_range_mut(2..2);
}

