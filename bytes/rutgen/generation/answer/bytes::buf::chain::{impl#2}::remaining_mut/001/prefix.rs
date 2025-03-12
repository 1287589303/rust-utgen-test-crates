// Answer 0

#[test]
fn test_remaining_mut_both_empty() {
    struct BufEmpty;

    unsafe impl BufMut for BufEmpty {
        fn remaining_mut(&self) -> usize {
            0
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
    }

    let buf_a = BufEmpty;
    let buf_b = BufEmpty;
    let chain = Chain { a: buf_a, b: buf_b };
    let result = chain.remaining_mut();
}

#[test]
fn test_remaining_mut_a_non_empty_b_empty() {
    struct BufA;

    unsafe impl BufMut for BufA {
        fn remaining_mut(&self) -> usize {
            3
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
    }

    struct BufB;

    unsafe impl BufMut for BufB {
        fn remaining_mut(&self) -> usize {
            0
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
    }

    let buf_a = BufA;
    let buf_b = BufB;
    let chain = Chain { a: buf_a, b: buf_b };
    let result = chain.remaining_mut();
}

#[test]
fn test_remaining_mut_a_empty_b_non_empty() {
    struct BufA;

    unsafe impl BufMut for BufA {
        fn remaining_mut(&self) -> usize {
            0
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
    }

    struct BufB;

    unsafe impl BufMut for BufB {
        fn remaining_mut(&self) -> usize {
            5
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
    }

    let buf_a = BufA;
    let buf_b = BufB;
    let chain = Chain { a: buf_a, b: buf_b };
    let result = chain.remaining_mut();
}

#[test]
fn test_remaining_mut_both_non_empty() {
    struct BufA;

    unsafe impl BufMut for BufA {
        fn remaining_mut(&self) -> usize {
            2
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
    }

    struct BufB;

    unsafe impl BufMut for BufB {
        fn remaining_mut(&self) -> usize {
            4
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
    }

    let buf_a = BufA;
    let buf_b = BufB;
    let chain = Chain { a: buf_a, b: buf_b };
    let result = chain.remaining_mut();
}

#[test]
fn test_remaining_mut_a_large_b_large() {
    struct BufA;

    unsafe impl BufMut for BufA {
        fn remaining_mut(&self) -> usize {
            usize::MAX
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
    }

    struct BufB;

    unsafe impl BufMut for BufB {
        fn remaining_mut(&self) -> usize {
            usize::MAX
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
    }

    let buf_a = BufA;
    let buf_b = BufB;
    let chain = Chain { a: buf_a, b: buf_b };
    let result = chain.remaining_mut();
}

