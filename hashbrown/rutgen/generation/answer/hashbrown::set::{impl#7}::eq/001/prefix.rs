// Answer 0

#[test]
fn test_eq_self_empty_other_non_empty() {
    let set_a: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder::new(), table: RawTable::new() } };
    let set_b: HashSet<i32> = HashSet {
        map: HashMap {
            hash_builder: DefaultHashBuilder::new(),
            table: RawTable::from_iter(vec![(1, ())].into_iter()),
        },
    };
    set_a.eq(&set_b);
}

#[test]
fn test_eq_self_non_empty_other_empty() {
    let set_a: HashSet<i32> = HashSet {
        map: HashMap {
            hash_builder: DefaultHashBuilder::new(),
            table: RawTable::from_iter(vec![(1, ())].into_iter()),
        },
    };
    let set_b: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder::new(), table: RawTable::new() } };
    set_a.eq(&set_b);
}

#[test]
fn test_eq_self_n_other_n_plus_one() {
    let set_a: HashSet<i32> = HashSet {
        map: HashMap {
            hash_builder: DefaultHashBuilder::new(),
            table: RawTable::from_iter(vec![(1, ()), (2, ())].into_iter()),
        },
    };
    let set_b: HashSet<i32> = HashSet {
        map: HashMap {
            hash_builder: DefaultHashBuilder::new(),
            table: RawTable::from_iter(vec![(1, ()), (2, ()), (3, ())].into_iter()),
        },
    };
    set_a.eq(&set_b);
}

#[test]
fn test_eq_self_n_plus_one_other_n() {
    let set_a: HashSet<i32> = HashSet {
        map: HashMap {
            hash_builder: DefaultHashBuilder::new(),
            table: RawTable::from_iter(vec![(1, ()), (2, ()), (3, ())].into_iter()),
        },
    };
    let set_b: HashSet<i32> = HashSet {
        map: HashMap {
            hash_builder: DefaultHashBuilder::new(),
            table: RawTable::from_iter(vec![(1, ()), (2, ())].into_iter()),
        },
    };
    set_a.eq(&set_b);
}

