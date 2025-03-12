// Answer 0

#[test]
fn test_set_vec_pos_max_bound() {
    struct MockBytesMut {
        data: *mut Shared,
        kind: usize,
    }

    impl MockBytesMut {
        unsafe fn set_vec_pos(&mut self, pos: usize) {
            debug_assert_eq!(self.kind, KIND_VEC);
            debug_assert!(pos <= MAX_VEC_POS);

            self.data = invalid_ptr((pos << VEC_POS_OFFSET) | (self.data as usize & NOT_VEC_POS_MASK));
        }
    }

    let mut mock = MockBytesMut {
        data: core::ptr::null_mut(),
        kind: KIND_VEC,
    };

    let pos = MAX_VEC_POS;
    unsafe {
        mock.set_vec_pos(pos);
    }
}

#[test]
#[should_panic]
fn test_set_vec_pos_exceeds_max_bound() {
    struct MockBytesMut {
        data: *mut Shared,
        kind: usize,
    }

    impl MockBytesMut {
        unsafe fn set_vec_pos(&mut self, pos: usize) {
            debug_assert_eq!(self.kind, KIND_VEC);
            debug_assert!(pos <= MAX_VEC_POS);

            self.data = invalid_ptr((pos << VEC_POS_OFFSET) | (self.data as usize & NOT_VEC_POS_MASK));
        }
    }

    let mut mock = MockBytesMut {
        data: core::ptr::null_mut(),
        kind: KIND_VEC,
    };

    let pos = MAX_VEC_POS + 1; // Exceeds the maximum valid position
    unsafe {
        mock.set_vec_pos(pos);
    }
}

