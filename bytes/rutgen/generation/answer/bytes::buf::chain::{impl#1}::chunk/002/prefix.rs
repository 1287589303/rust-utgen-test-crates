// Answer 0

#[test]
fn test_chunk_with_a_empty_and_b_non_empty() {
    struct EmptyBuf;
    impl Buf for EmptyBuf {
        fn remaining(&self) -> usize {
            0
        }
        fn chunk(&self) -> &[u8] {
            &[]
        }
        fn advance(&mut self, _: usize) {}
    }

    struct NonEmptyBuf {
        data: &'static [u8],
    }
    impl Buf for NonEmptyBuf {
        fn remaining(&self) -> usize {
            self.data.len()
        }
        fn chunk(&self) -> &[u8] {
            self.data
        }
        fn advance(&mut self, _: usize) {}
    }

    let a = EmptyBuf;
    let b = NonEmptyBuf { data: &[1, 2, 3] };
    let chain = Chain { a, b };

    let result = chain.chunk();
}

#[test]
fn test_chunk_with_a_empty_and_b_single_byte() {
    struct EmptyBuf;
    impl Buf for EmptyBuf {
        fn remaining(&self) -> usize {
            0
        }
        fn chunk(&self) -> &[u8] {
            &[]
        }
        fn advance(&mut self, _: usize) {}
    }

    struct SingleByteBuf {
        data: &'static [u8],
    }
    impl Buf for SingleByteBuf {
        fn remaining(&self) -> usize {
            self.data.len()
        }
        fn chunk(&self) -> &[u8] {
            self.data
        }
        fn advance(&mut self, _: usize) {}
    }

    let a = EmptyBuf;
    let b = SingleByteBuf { data: &[42] };
    let chain = Chain { a, b };

    let result = chain.chunk();
}

#[test]
fn test_chunk_with_a_empty_and_b_multiple_bytes() {
    struct EmptyBuf;
    impl Buf for EmptyBuf {
        fn remaining(&self) -> usize {
            0
        }
        fn chunk(&self) -> &[u8] {
            &[]
        }
        fn advance(&mut self, _: usize) {}
    }

    struct MultipleBytesBuf {
        data: &'static [u8],
    }
    impl Buf for MultipleBytesBuf {
        fn remaining(&self) -> usize {
            self.data.len()
        }
        fn chunk(&self) -> &[u8] {
            self.data
        }
        fn advance(&mut self, _: usize) {}
    }

    let a = EmptyBuf;
    let b = MultipleBytesBuf { data: &[10, 20, 30, 40] };
    let chain = Chain { a, b };

    let result = chain.chunk();
}

