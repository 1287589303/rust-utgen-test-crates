// Answer 0

#[test]
fn test_once_ref_set_and_get() {
    let mut once_ref = OnceRef::<i32>::new();
    let value = 42;
    once_ref.set(&value).unwrap();
    let retrieved = once_ref.get().unwrap();
}

#[test]
fn test_once_ref_get_or_init() {
    let once_ref = OnceRef::<i32>::new();
    let value = once_ref.get_or_init(|| &42);
    assert_eq!(*value, 42);
}

#[test]
fn test_once_ref_get_or_try_init() {
    let once_ref = OnceRef::<i32>::new();
    let value = once_ref.get_or_try_init(|| Ok(&100)).unwrap();
    assert_eq!(*value, 100);
}

#[test]
#[should_panic]
fn test_once_ref_dangling_reference() {
    let mut l = OnceRef::<i32>::new();
    {
        let y = 2;
        let mut r = OnceRef::<i32>::new();
        r.set(&y).unwrap();
        core::mem::swap(&mut l, &mut r);
    }
    eprintln!("uaf: {}", l.get().unwrap());
}

#[test]
fn test_once_ref_multiple_initializations() {
    let once_ref = OnceRef::<i32>::new();
    let first_value = once_ref.get_or_init(|| &15);
    let second_value = once_ref.get_or_init(|| &30); // Should return the first_value
    assert_eq!(*first_value, *second_value);
}

#[test]
fn test_once_ref_reinitialize() {
    let once_ref = OnceRef::<i32>::new();
    let value1 = 20;
    let value2 = 30;

    once_ref.set(&value1).unwrap();
    assert_eq!(*once_ref.get().unwrap(), 20);

    once_ref.set(&value2).unwrap();
    assert_eq!(*once_ref.get().unwrap(), 30);
}

