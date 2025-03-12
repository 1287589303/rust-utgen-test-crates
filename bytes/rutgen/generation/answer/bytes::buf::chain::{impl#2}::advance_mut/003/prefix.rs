// Answer 0

#[test]
fn test_advance_mut_a_rem_is_zero_with_zero_cnt() {
    struct BufImpl {
        remaining: usize,
    }

    unsafe impl BufMut for BufImpl {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        unsafe fn advance_mut(&mut self, _cnt: usize) {
            // Do nothing
        }
    }

    let a = BufImpl { remaining: 0 };
    let b = BufImpl { remaining: 0 };
    let mut chain = Chain { a, b };

    unsafe {
        chain.advance_mut(0);
    }
}

#[test]
fn test_advance_mut_a_rem_is_zero_with_non_zero_cnt() {
    struct BufImpl {
        remaining: usize,
    }

    unsafe impl BufMut for BufImpl {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        unsafe fn advance_mut(&mut self, _cnt: usize) {
            // Do nothing
        }
    }

    let a = BufImpl { remaining: 0 };
    let b = BufImpl { remaining: 2 };
    let mut chain = Chain { a, b };

    unsafe {
        chain.advance_mut(1);
    }
}

#[test]
fn test_advance_mut_a_rem_is_zero_with_cnt_greater_than_remaining() {
    struct BufImpl {
        remaining: usize,
    }

    unsafe impl BufMut for BufImpl {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        unsafe fn advance_mut(&mut self, _cnt: usize) {
            // Do nothing
        }
    }

    let a = BufImpl { remaining: 0 };
    let b = BufImpl { remaining: 3 };
    let mut chain = Chain { a, b };

    unsafe {
        chain.advance_mut(4);
    }
} 

#[test]
fn test_advance_mut_a_rem_is_zero_with_cnt_equal_to_remaining_plus_one() {
    struct BufImpl {
        remaining: usize,
    }

    unsafe impl BufMut for BufImpl {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        unsafe fn advance_mut(&mut self, _cnt: usize) {
            // Do nothing
        }
    }

    let a = BufImpl { remaining: 0 };
    let b = BufImpl { remaining: 1 };
    let mut chain = Chain { a, b };

    unsafe {
        chain.advance_mut(1);
    }
} 

#[test]
fn test_advance_mut_a_rem_is_non_zero_with_cnt_equal_to_remaining_plus_n() {
    struct BufImpl {
        remaining: usize,
    }

    unsafe impl BufMut for BufImpl {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        unsafe fn advance_mut(&mut self, _cnt: usize) {
            // Do nothing
        }
    }

    let a = BufImpl { remaining: 1 }; // a's remaining is > 0
    let b = BufImpl { remaining: 1 };
    let mut chain = Chain { a, b };

    unsafe {
        chain.advance_mut(2);
    }
}

