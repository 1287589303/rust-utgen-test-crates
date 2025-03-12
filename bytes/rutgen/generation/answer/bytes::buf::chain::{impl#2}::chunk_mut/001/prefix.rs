// Answer 0

#[test]
fn test_chunk_mut_a_has_remaining_mut() {
    struct TestBufMutA {
        remaining: usize,
    }

    struct TestBufMutB {
        remaining: usize,
    }

    unsafe impl BufMut for TestBufMutA {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice([std::mem::MaybeUninit::uninit(); 10])
        }
    }

    unsafe impl BufMut for TestBufMutB {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice([std::mem::MaybeUninit::uninit(); 10])
        }
    }

    let buf_a = TestBufMutA { remaining: 5 };
    let buf_b = TestBufMutB { remaining: 0 };
    let mut chain = Chain { a: buf_a, b: buf_b };
    let _result = chain.chunk_mut();
}

