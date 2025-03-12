// Answer 0

#[test]
fn test_chain_mut_advance_mut_case_1() {
    struct BufA {
        data: Vec<u8>,
        pos: usize,
    }

    impl BufMut for BufA {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            todo!()
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }
    }

    struct BufB {
        data: Vec<u8>,
        pos: usize,
    }

    impl BufMut for BufB {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            todo!()
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }
    }

    let a = BufA { data: vec![1, 2, 3], pos: 0 };
    let b = BufB { data: vec![4, 5, 6, 7], pos: 0 };
    let mut chain = Chain { a, b };

    unsafe {
        chain.advance_mut(5);
    }
}

#[test]
fn test_chain_mut_advance_mut_case_2() {
    struct BufA {
        data: Vec<u8>,
        pos: usize,
    }

    impl BufMut for BufA {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            todo!()
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }
    }

    struct BufB {
        data: Vec<u8>,
        pos: usize,
    }

    impl BufMut for BufB {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            todo!()
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }
    }

    let a = BufA { data: vec![1, 2], pos: 0 };
    let b = BufB { data: vec![3, 4, 5, 6, 7], pos: 0 };
    let mut chain = Chain { a, b };

    unsafe {
        chain.advance_mut(3);
    }
}

