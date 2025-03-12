// Answer 0

#[test]
fn test_difference_next_with_elements_present_in_other() {
    let mut hash_set = HashSet::new();
    hash_set.map.insert("a", ());
    hash_set.map.insert("b", ());

    let mut iter = Iter { inner: RawIter::new(vec!["a", "b"]), marker: PhantomData };
    let other = &hash_set;

    let mut difference = Difference { iter, other };

    let result = difference.next();
}

#[test]
fn test_difference_next_with_elements_absent_in_other() {
    let mut hash_set = HashSet::new();
    hash_set.map.insert("a", ());
    hash_set.map.insert("b", ());

    let mut iter = Iter { inner: RawIter::new(vec!["c", "d"]), marker: PhantomData };
    let other = &hash_set;

    let mut difference = Difference { iter, other };

    let result = difference.next();
}

#[test]
fn test_difference_next_with_empty_iter() {
    let mut hash_set = HashSet::new();
    hash_set.map.insert("a", ());

    let mut iter = Iter { inner: RawIter::new(vec![]), marker: PhantomData };
    let other = &hash_set;

    let mut difference = Difference { iter, other };

    let result = difference.next();
}

