// Answer 0

#[test]
fn test_clone_from_left_to_left() {
    struct TestStruct {
        value: i32,
    }

    let mut source = Either::Left(TestStruct { value: 10 });
    let mut dest = Either::Left(TestStruct { value: 20 });
    
    dest.clone_from(&source);
}

#[test]
fn test_clone_from_left_to_left_different_values() {
    struct TestStruct {
        value: i32,
    }

    let mut source = Either::Left(TestStruct { value: 30 });
    let mut dest = Either::Left(TestStruct { value: 40 });

    dest.clone_from(&source);
}

