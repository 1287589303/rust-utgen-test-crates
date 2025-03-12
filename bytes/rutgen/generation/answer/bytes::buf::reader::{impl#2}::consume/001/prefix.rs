// Answer 0

#[test]
fn test_consume_zero_amt() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_slice(&mut self, dst: &mut [u8]) {
            let to_copy = cmp::min(self.remaining(), dst.len());
            dst[..to_copy].copy_from_slice(&self.data[self.pos..self.pos + to_copy]);
            self.advance(to_copy);
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.pos];
            self.advance(1);
            byte
        }
        // Other methods omitted for brevity
    }

    let buf = TestBuf { data: vec![1, 2, 3, 4], pos: 0 };
    let mut reader = Reader { buf };

    reader.consume(0);
}

#[test]
fn test_consume_full_amt() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_slice(&mut self, dst: &mut [u8]) {
            let to_copy = cmp::min(self.remaining(), dst.len());
            dst[..to_copy].copy_from_slice(&self.data[self.pos..self.pos + to_copy]);
            self.advance(to_copy);
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.pos];
            self.advance(1);
            byte
        }
        // Other methods omitted for brevity
    }

    let buf = TestBuf { data: vec![1, 2, 3, 4], pos: 0 };
    let mut reader = Reader { buf };

    let amt = reader.buf.remaining();
    reader.consume(amt);
}

#[test]
fn test_consume_partial_amt() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_slice(&mut self, dst: &mut [u8]) {
            let to_copy = cmp::min(self.remaining(), dst.len());
            dst[..to_copy].copy_from_slice(&self.data[self.pos..self.pos + to_copy]);
            self.advance(to_copy);
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.pos];
            self.advance(1);
            byte
        }
        // Other methods omitted for brevity
    }

    let buf = TestBuf { data: vec![1, 2, 3, 4], pos: 0 };
    let mut reader = Reader { buf };

    let amt = reader.buf.remaining() / 2;
    reader.consume(amt);
}

#[test]
fn test_consume_over_amt() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_slice(&mut self, dst: &mut [u8]) {
            let to_copy = cmp::min(self.remaining(), dst.len());
            dst[..to_copy].copy_from_slice(&self.data[self.pos..self.pos + to_copy]);
            self.advance(to_copy);
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.pos];
            self.advance(1);
            byte
        }
        // Other methods omitted for brevity
    }

    let buf = TestBuf { data: vec![1, 2, 3, 4], pos: 0 };
    let mut reader = Reader { buf };

    let amt = reader.buf.remaining() + 1; // This will cause an out-of-bounds access
    reader.consume(amt);
}

