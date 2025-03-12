// Answer 0

#[test]
fn test_size_hint_empty_u32() {
    let indices = IndexVecIntoIter::U32(vec::IntoIter::new(vec![]));
    let iter = SliceChooseIter {
        slice: &[],
        _phantom: core::marker::PhantomData,
        indices,
    };
    iter.size_hint();
}

#[test]
fn test_size_hint_single_u32() {
    let indices = IndexVecIntoIter::U32(vec::IntoIter::new(vec![0]));
    let iter = SliceChooseIter {
        slice: &[10],
        _phantom: core::marker::PhantomData,
        indices,
    };
    iter.size_hint();
}

#[test]
fn test_size_hint_multiple_u32() {
    let indices = IndexVecIntoIter::U32(vec::IntoIter::new(vec![0, 1, 2]));
    let iter = SliceChooseIter {
        slice: &[10, 20, 30],
        _phantom: core::marker::PhantomData,
        indices,
    };
    iter.size_hint();
}

#[test]
fn test_size_hint_empty_u64() {
    #[cfg(target_pointer_width = "64")]
    {
        let indices = IndexVecIntoIter::U64(vec::IntoIter::new(vec![]));
        let iter = SliceChooseIter {
            slice: &[],
            _phantom: core::marker::PhantomData,
            indices,
        };
        iter.size_hint();
    }
}

#[test]
fn test_size_hint_single_u64() {
    #[cfg(target_pointer_width = "64")]
    {
        let indices = IndexVecIntoIter::U64(vec::IntoIter::new(vec![0]));
        let iter = SliceChooseIter {
            slice: &[10],
            _phantom: core::marker::PhantomData,
            indices,
        };
        iter.size_hint();
    }
}

#[test]
fn test_size_hint_multiple_u64() {
    #[cfg(target_pointer_width = "64")]
    {
        let indices = IndexVecIntoIter::U64(vec::IntoIter::new(vec![0, 1, 2]));
        let iter = SliceChooseIter {
            slice: &[10, 20, 30],
            _phantom: core::marker::PhantomData,
            indices,
        };
        iter.size_hint();
    }
}

