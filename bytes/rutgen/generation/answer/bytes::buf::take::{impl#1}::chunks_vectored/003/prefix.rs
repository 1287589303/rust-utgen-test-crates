// Answer 0

#[test]
fn test_chunks_vectored_limit_zero() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            self.data[..len].to_vec().into()
        }

        #[cfg(feature = "std")]
        fn chunks_vectored<'a>(&'a self, dst: &mut [std::io::IoSlice<'a>]) -> usize {
            let slice = &self.data[self.position..];
            let cnt = slice.chunks(dst.len()).take(1).count();
            for (i, dst_slice) in dst.iter_mut().enumerate().take(cnt) {
                *dst_slice = std::io::IoSlice::new(slice);
            }
            cnt
        }
    }

    let inner_buf = TestBuf { data: vec![1, 2, 3, 4, 5], position: 0 };
    let take_buf = Take { inner: inner_buf, limit: 0 };

    let mut dst: [std::io::IoSlice; 4] = [
        std::io::IoSlice::new(&[]),
        std::io::IoSlice::new(&[]),
        std::io::IoSlice::new(&[]),
        std::io::IoSlice::new(&[]),
    ];

    let result = take_buf.chunks_vectored(&mut dst);
}

#[test]
fn test_chunks_vectored_non_empty_dst() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            self.data[..len].to_vec().into()
        }

        #[cfg(feature = "std")]
        fn chunks_vectored<'a>(&'a self, dst: &mut [std::io::IoSlice<'a>]) -> usize {
            let slice = &self.data[self.position..];
            let cnt = slice.chunks(dst.len()).take(1).count();
            for (i, dst_slice) in dst.iter_mut().enumerate().take(cnt) {
                *dst_slice = std::io::IoSlice::new(slice);
            }
            cnt
        }
    }

    let inner_buf = TestBuf { data: vec![1, 2, 3, 4, 5], position: 0 };
    let take_buf = Take { inner: inner_buf, limit: 5 };

    let mut dst: [std::io::IoSlice; 4] = [
        std::io::IoSlice::new(&[0; 5]),
        std::io::IoSlice::new(&[0; 5]),
        std::io::IoSlice::new(&[0; 5]),
        std::io::IoSlice::new(&[0; 5]),
    ];

    let result = take_buf.chunks_vectored(&mut dst);
}

