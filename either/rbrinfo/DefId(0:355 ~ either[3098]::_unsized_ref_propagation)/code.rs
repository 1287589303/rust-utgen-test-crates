fn _unsized_ref_propagation() {
    check_t!(str);

    fn check_array_ref<T: AsRef<[Item]>, Item>() {}
    fn check_array_mut<T: AsMut<[Item]>, Item>() {}

    fn propagate_array_ref<T1: AsRef<[Item]>, T2: AsRef<[Item]>, Item>() {
        check_array_ref::<Either<T1, T2>, _>()
    }

    fn propagate_array_mut<T1: AsMut<[Item]>, T2: AsMut<[Item]>, Item>() {
        check_array_mut::<Either<T1, T2>, _>()
    }
}