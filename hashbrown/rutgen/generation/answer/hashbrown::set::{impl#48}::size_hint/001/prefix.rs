// Answer 0

#[test]
fn size_hint_empty_other() {
    let other: HashSet<i32> = HashSet { map: HashMap::default() }; 
    let iter: Iter<i32> = Iter { iter: Keys::default() }; 
    let difference = Difference { iter, other: &other };
    difference.size_hint();
}

#[test]
fn size_hint_non_empty_other_lower_greater_than_len() {
    let other: HashSet<i32> = HashSet { map: HashMap::from_iter(vec![(1, ()), (2, ())]) }; 
    let iter: Iter<i32> = Iter { iter: Keys::from(vec![3, 4, 5].into_iter()) }; 
    let difference = Difference { iter, other: &other };
    difference.size_hint();
}

#[test]
fn size_hint_non_empty_other_lower_equal_len() {
    let other: HashSet<i32> = HashSet { map: HashMap::from_iter(vec![(1, ()), (2, ())]) }; 
    let iter: Iter<i32> = Iter { iter: Keys::from(vec![3, 4].into_iter()) }; 
    let difference = Difference { iter, other: &other };
    difference.size_hint();
}

#[test]
fn size_hint_other_greater_than_iter() {
    let other: HashSet<i32> = HashSet { map: HashMap::from_iter(vec![(1, ()), (2, ()), (3, ())]) }; 
    let iter: Iter<i32> = Iter { iter: Keys::from(vec![4].into_iter()) }; 
    let difference = Difference { iter, other: &other };
    difference.size_hint();
}

#[test]
fn size_hint_iter_when_other_empty() {
    let other: HashSet<i32> = HashSet { map: HashMap::default() }; 
    let iter: Iter<i32> = Iter { iter: Keys::from(vec![1, 2, 3].into_iter()) }; 
    let difference = Difference { iter, other: &other };
    difference.size_hint();
}

#[test]
fn size_hint_iter_equal_other() {
    let other: HashSet<i32> = HashSet { map: HashMap::from_iter(vec![(1, ()), (2, ())]) }; 
    let iter: Iter<i32> = Iter { iter: Keys::from(vec![1, 2].into_iter()) }; 
    let difference = Difference { iter, other: &other };
    difference.size_hint();
}

