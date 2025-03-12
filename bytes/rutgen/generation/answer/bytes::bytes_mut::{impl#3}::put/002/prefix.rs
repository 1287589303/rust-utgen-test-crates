// Answer 0

#[test]
fn test_put_with_empty_buf() {
    struct EmptyBuf;

    impl Buf for EmptyBuf {
        fn remaining(&self) -> usize {
            0
        }

        fn chunk(&self) -> &[u8] {
            &[]
        }

        fn advance(&mut self, _cnt: usize) {}
    }

    let mut bytes_mut = BytesMut::new();
    let src = EmptyBuf;
    bytes_mut.put(src);
}

#[test]
fn test_put_with_empty_slice() {
    struct EmptySliceBuf<'a> {
        data: &'a [u8],
    }

    impl<'a> Buf for EmptySliceBuf<'a> {
        fn remaining(&self) -> usize {
            0
        }

        fn chunk(&self) -> &[u8] {
            self.data
        }

        fn advance(&mut self, _cnt: usize) {}
    }

    let empty_slice: &[u8] = &[];
    let mut bytes_mut = BytesMut::new();
    let src = EmptySliceBuf { data: empty_slice };
    bytes_mut.put(src);
}

