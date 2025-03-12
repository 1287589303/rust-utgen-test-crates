// Answer 0

#[test]
fn test_push_empty_vec() {
    let mut vec = vec![];
    let mut rc_vec_mut = RcVecMut { inner: &mut vec };
    rc_vec_mut.push(1);
}

#[test]
fn test_push_single_element() {
    let mut vec = vec![10];
    let mut rc_vec_mut = RcVecMut { inner: &mut vec };
    rc_vec_mut.push(20);
}

#[test]
fn test_push_multiple_elements() {
    let mut vec = vec![1, 2, 3];
    let mut rc_vec_mut = RcVecMut { inner: &mut vec };
    rc_vec_mut.push(4);
    rc_vec_mut.push(5);
}

#[test]
fn test_push_string() {
    let mut vec = vec!["hello".to_string()];
    let mut rc_vec_mut = RcVecMut { inner: &mut vec };
    rc_vec_mut.push("world".to_string());
}

#[test]
fn test_push_struct() {
    struct MyStruct {
        value: i32,
    }
    let mut vec = vec![MyStruct { value: 0 }];
    let mut rc_vec_mut = RcVecMut { inner: &mut vec };
    rc_vec_mut.push(MyStruct { value: 1 });
}

