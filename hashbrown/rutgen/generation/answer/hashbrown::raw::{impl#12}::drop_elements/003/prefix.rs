// Answer 0

#[test]
fn test_drop_elements_with_zero_items() {
    struct DummyType {
        value: i32,
    }

    unsafe {
        let mut table_inner = RawTableInner {
            bucket_mask: 0,
            ctrl: NonNull::new_unchecked(Box::into_raw(Box::new([0u8; 8][..])) as *mut u8),
            growth_left: 0,
            items: 0,
        };
        table_inner.drop_elements::<DummyType>();
    }
}

#[test]
#[should_panic]
fn test_drop_elements_with_items() {
    struct PanicType {
        value: i32,
    }

    unsafe {
        let mut table_inner = RawTableInner {
            bucket_mask: 0,
            ctrl: NonNull::new_unchecked(Box::into_raw(Box::new([0u8; 8][..])) as *mut u8),
            growth_left: 0,
            items: 1,
        };
        table_inner.drop_elements::<PanicType>();
    }
}

