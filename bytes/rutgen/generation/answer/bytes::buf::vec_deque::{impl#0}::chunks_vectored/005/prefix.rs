// Answer 0

#[test]
fn test_chunks_vectored_with_two_slices() {
    struct TestBuf {
        inner: VecDeque<u8>,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.inner.len()
        }

        fn chunk(&self) -> &[u8] {
            self.inner.as_slices().0
        }

        fn advance(&mut self, cnt: usize) {
            for _ in 0..cnt {
                self.inner.pop_front();
            }
        }
    }

    let mut buf = TestBuf { 
        inner: VecDeque::from(vec![1, 2, 3, 4, 5, 6]) 
    };
    let mut dst = vec![io::IoSlice::new(&[]); 2]; // preallocate space for two slices
    let result = buf.chunks_vectored(&mut dst);
}

