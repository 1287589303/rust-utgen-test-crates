// Answer 0

#[test]
fn test_fill_bytes_via_next_left_len_eq_8() {
    struct DummyRng;

    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn jump(&mut self) {}
    }

    let mut rng = DummyRng;
    let mut dest = [0u8; 8];
    fill_bytes_via_next(&mut rng, &mut dest);
}

#[test]
fn test_fill_bytes_via_next_left_len_lt_8() {
    struct DummyRng;

    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn jump(&mut self) {}
    }

    let mut rng = DummyRng;
    let mut dest = [0u8; 7];
    fill_bytes_via_next(&mut rng, &mut dest);
}

#[test]
fn test_fill_bytes_via_next_n_eq_4() {
    struct DummyRng;

    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn jump(&mut self) {}
    }

    let mut rng = DummyRng;
    let mut dest = [0u8; 4];
    fill_bytes_via_next(&mut rng, &mut dest);
}

#[test]
fn test_fill_bytes_via_next_n_eq_0() {
    struct DummyRng;

    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn jump(&mut self) {}
    }

    let mut rng = DummyRng;
    let mut dest: [u8; 0] = [];
    fill_bytes_via_next(&mut rng, &mut dest);
}

