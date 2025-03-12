// Answer 0

#[test]
fn test_sorted_by_with_single_element() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32, String>> {
            self.entries
        }

        fn sorted_by<F>(self, mut cmp: F) -> IntoIter<i32, String>
        where
            F: FnMut(&i32, &String, &i32, &String) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            IntoIter::new(entries)
        }
    }

    let map = TestMap { 
        entries: vec![Bucket { hash: HashValue::default(), key: 1, value: "a".to_string() }]
    };

    let _iter = map.sorted_by(|k1, v1, k2, v2| k1.cmp(k2));
}

#[test]
fn test_sorted_by_with_empty_map() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32, String>> {
            self.entries
        }

        fn sorted_by<F>(self, mut cmp: F) -> IntoIter<i32, String>
        where
            F: FnMut(&i32, &String, &i32, &String) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            IntoIter::new(entries)
        }
    }

    let map = TestMap { entries: vec![] };

    let _iter = map.sorted_by(|k1, v1, k2, v2| k1.cmp(k2));
}

#[test]
fn test_sorted_by_with_large_map() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32, String>> {
            self.entries
        }

        fn sorted_by<F>(self, mut cmp: F) -> IntoIter<i32, String>
        where
            F: FnMut(&i32, &String, &i32, &String) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            IntoIter::new(entries)
        }
    }

    let entries: Vec<Bucket<i32, String>> = (0..1000).map(|i| Bucket { hash: HashValue::default(), key: i, value: format!("value {}", i) }).collect();
    let map = TestMap { entries };

    let _iter = map.sorted_by(|k1, v1, k2, v2| k1.cmp(k2));
}

#[test]
fn test_sorted_by_with_duplicate_keys() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32, String>> {
            self.entries
        }

        fn sorted_by<F>(self, mut cmp: F) -> IntoIter<i32, String>
        where
            F: FnMut(&i32, &String, &i32, &String) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            IntoIter::new(entries)
        }
    }

    let map = TestMap { 
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: "b".to_string() },
            Bucket { hash: HashValue::default(), key: 1, value: "a".to_string() },
        ]
    };

    let _iter = map.sorted_by(|k1, v1, k2, v2| k1.cmp(k2).then(v1.cmp(v2)));
}

#[test]
fn test_sorted_by_with_different_key_value_types() {
    struct TestMap {
        entries: Vec<Bucket<String, f64>>,
    }

    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<String, f64>> {
            self.entries
        }

        fn sorted_by<F>(self, mut cmp: F) -> IntoIter<String, f64>
        where
            F: FnMut(&String, &f64, &String, &f64) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            IntoIter::new(entries)
        }
    }

    let map = TestMap {
        entries: vec![
            Bucket { hash: HashValue::default(), key: "c".to_string(), value: 3.0 },
            Bucket { hash: HashValue::default(), key: "a".to_string(), value: 1.0 },
            Bucket { hash: HashValue::default(), key: "b".to_string(), value: 2.0 },
        ]
    };

    let _iter = map.sorted_by(|k1, v1, k2, v2| k1.cmp(k2));
}

