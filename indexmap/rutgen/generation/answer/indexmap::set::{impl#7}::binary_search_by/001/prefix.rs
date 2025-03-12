// Answer 0

#[test]
fn test_binary_search_by_found() {
    struct TestSet {
        data: Vec<i32>,
    }

    let mut set = TestSet { data: vec![1, 2, 3, 4, 5] };

    let result = set.binary_search_by(|&x| {
        if x < 3 {
            Ordering::Less
        } else if x > 3 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
}

#[test]
fn test_binary_search_by_not_found() {
    struct TestSet {
        data: Vec<i32>,
    }

    let mut set = TestSet { data: vec![1, 2, 4, 5] };

    let result = set.binary_search_by(|&x| {
        if x < 3 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

#[test]
fn test_binary_search_by_insert_position() {
    struct TestSet {
        data: Vec<i32>,
    }

    let mut set = TestSet { data: vec![1, 2, 3, 5] };

    let result = set.binary_search_by(|&x| {
        if x < 4 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

#[test]
fn test_binary_search_by_empty_set() {
    struct TestSet {
        data: Vec<i32>,
    }

    let mut set = TestSet { data: vec![] };

    let result = set.binary_search_by(|&x| {
        if x < 1 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

#[test]
fn test_binary_search_by_single_element_found() {
    struct TestSet {
        data: Vec<i32>,
    }

    let mut set = TestSet { data: vec![3] };

    let result = set.binary_search_by(|&x| {
        if x < 3 {
            Ordering::Less
        } else if x > 3 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
}

#[test]
fn test_binary_search_by_single_element_not_found() {
    struct TestSet {
        data: Vec<i32>,
    }

    let mut set = TestSet { data: vec![3] };

    let result = set.binary_search_by(|&x| {
        if x < 2 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

