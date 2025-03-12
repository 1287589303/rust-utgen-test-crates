// Answer 0

#[test]
fn test_propagate_array_ref_with_arrays() {
    struct ArrayRef1<'a>(&'a [u8]);
    struct ArrayRef2<'a>(&'a [i32]);

    fn check_array_ref<T: AsRef<[Item]>, Item>() {}

    fn propagate_array_ref<T1: AsRef<[Item]>, T2: AsRef<[Item]>, Item>() {
        check_array_ref::<Either<T1, T2>, _>()
    }

    propagate_array_ref::<ArrayRef1, ArrayRef2, u8>();
}

#[test]
fn test_propagate_array_ref_with_mixed_types() {
    struct ArrayRef1<'a>(&'a [u8]);
    struct ArrayRef2<'a>(&'a [f64]);

    fn check_array_ref<T: AsRef<[Item]>, Item>() {}

    fn propagate_array_ref<T1: AsRef<[Item]>, T2: AsRef<[Item]>, Item>() {
        check_array_ref::<Either<T1, T2>, _>()
    }

    propagate_array_ref::<ArrayRef1, ArrayRef2, f64>();
}

#[test]
fn test_propagate_array_mut_with_arrays() {
    struct ArrayMut1<'a>(&'a mut [u8]);
    struct ArrayMut2<'a>(&'a mut [i32]);

    fn check_array_mut<T: AsMut<[Item]>, Item>() {}

    fn propagate_array_mut<T1: AsMut<[Item]>, T2: AsMut<[Item]>, Item>() {
        check_array_mut::<Either<T1, T2>, _>()
    }

    propagate_array_mut::<ArrayMut1, ArrayMut2, u8>();
}

#[test]
fn test_propagate_array_mut_with_mixed_types() {
    struct ArrayMut1<'a>(&'a mut [u8]);
    struct ArrayMut2<'a>(&'a mut [f64]);

    fn check_array_mut<T: AsMut<[Item]>, Item>() {}

    fn propagate_array_mut<T1: AsMut<[Item]>, T2: AsMut<[Item]>, Item>() {
        check_array_mut::<Either<T1, T2>, _>()
    }

    propagate_array_mut::<ArrayMut1, ArrayMut2, f64>();
}

#[test]
fn test_propagate_array_with_empty() {
    struct ArrayRef1<'a>(&'a [u8]);
    struct ArrayRef2<'a>(&'a [i32]);

    fn check_array_ref<T: AsRef<[Item]>, Item>() {}

    fn propagate_array_ref<T1: AsRef<[Item]>, T2: AsRef<[Item]>, Item>() {
        check_array_ref::<Either<T1, T2>, _>()
    }

    let empty_u8: &[u8] = &[];
    let empty_i32: &[i32] = &[];

    propagate_array_ref::<ArrayRef1, ArrayRef2, u8>();
}

