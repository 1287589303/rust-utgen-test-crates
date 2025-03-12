// Answer 0

#[test]
fn test_index_zero() {
    struct TestStruct {
        data: Vec<i32>,
    }

    let data = (0..=SmallIndex::MAX.as_usize()).map(|i| i as i32).collect();
    let test_struct = TestStruct { data };

    let index = SmallIndex::ZERO;
    let _result = test_struct.index(index);
}

#[test]
fn test_index_max() {
    struct TestStruct {
        data: Vec<i32>,
    }

    let data = (0..=SmallIndex::MAX.as_usize()).map(|i| i as i32).collect();
    let test_struct = TestStruct { data };

    let index = SmallIndex::MAX;
    let _result = test_struct.index(index);
}

#[test]
fn test_index_mid() {
    struct TestStruct {
        data: Vec<i32>,
    }

    let data = (0..=SmallIndex::MAX.as_usize()).map(|i| i as i32).collect();
    let test_struct = TestStruct { data };

    let index = SmallIndex::new((SmallIndex::MAX.as_usize() / 2)).unwrap();
    let _result = test_struct.index(index);
}

#[test]
fn test_index_one_more() {
    struct TestStruct {
        data: Vec<i32>,
    }

    let data = (0..=SmallIndex::MAX.as_usize()).map(|i| i as i32).collect();
    let test_struct = TestStruct { data };

    let index = SmallIndex::must(SmallIndex::MAX.as_usize() + 1);
    let _result = test_struct.index(index);
}

