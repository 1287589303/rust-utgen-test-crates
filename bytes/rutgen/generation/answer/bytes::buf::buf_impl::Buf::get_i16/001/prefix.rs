// Answer 0

#[test]
fn test_get_i16_valid() {
    struct TestBuf {
        data: &'static [u8],
        pos: usize,
    }
    
    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }
        
        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }
        
        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }
        
        fn get_i16(&mut self) -> i16 {
            let bytes = [self.chunk()[0], self.chunk()[1]];
            self.advance(2);
            i16::from_be_bytes(bytes)
        }
    }
    
    let mut buf = TestBuf { data: &b"\x08\x09 hello"[..], pos: 0 };
    let result = buf.get_i16();
}

#[test]
fn test_get_i16_edge_case() {
    struct TestBuf {
        data: &'static [u8],
        pos: usize,
    }
    
    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }
        
        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }
        
        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }
        
        fn get_i16(&mut self) -> i16 {
            let bytes = [self.chunk()[0], self.chunk()[1]];
            self.advance(2);
            i16::from_be_bytes(bytes)
        }
    }

    let mut buf = TestBuf { data: &b"\x00\x01"[..], pos: 0 };
    let result = buf.get_i16();
    
    let mut buf_neg = TestBuf { data: &b"\x80\x00"[..], pos: 0 };
    let result_neg = buf_neg.get_i16();
}

#[test]
fn test_get_i16_large_buffer() {
    struct TestBuf {
        data: &'static [u8],
        pos: usize,
    }
    
    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }
        
        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }
        
        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }
        
        fn get_i16(&mut self) -> i16 {
            let bytes = [self.chunk()[0], self.chunk()[1]];
            self.advance(2);
            i16::from_be_bytes(bytes)
        }
    }

    let mut buf = TestBuf { data: &b"\x12\x34 more data than needed"[..], pos: 0 };
    let result = buf.get_i16();
}

#[test]
#[should_panic]
fn test_get_i16_not_enough_data() {
    struct TestBuf {
        data: &'static [u8],
        pos: usize,
    }
    
    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }
        
        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }
        
        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }
        
        fn get_i16(&mut self) -> i16 {
            if self.remaining() < 2 {
                panic!("not enough data");
            }
            let bytes = [self.chunk()[0], self.chunk()[1]];
            self.advance(2);
            i16::from_be_bytes(bytes)
        }
    }

    let mut buf = TestBuf { data: &b"\x01"[..], pos: 0 };
    let result = buf.get_i16();
}

