// Answer 0

#[test]
fn test_get_mut_with_zero_limit() {
    struct SimpleBuf {
        data: Vec<u8>,
        pos: usize,
    }
    
    impl SimpleBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }
        
        fn advance(&mut self, n: usize) {
            self.pos += n;
        }
    }
    
    let mut buf = Take {
        inner: SimpleBuf::new(b"hello world".to_vec()),
        limit: 0,
    };
    
    let inner = buf.get_mut();
    inner.advance(0);
}

#[test]
fn test_get_mut_with_full_limit() {
    struct SimpleBuf {
        data: Vec<u8>,
        pos: usize,
    }
    
    impl SimpleBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }
        
        fn advance(&mut self, n: usize) {
            self.pos += n;
        }
    }
    
    let mut buf = Take {
        inner: SimpleBuf::new(b"hello world".to_vec()),
        limit: 11,
    };
    
    let inner = buf.get_mut();
    inner.advance(11);
}

#[test]
fn test_get_mut_with_partial_limit() {
    struct SimpleBuf {
        data: Vec<u8>,
        pos: usize,
    }
    
    impl SimpleBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }
        
        fn advance(&mut self, n: usize) {
            self.pos += n;
        }
    }
    
    let mut buf = Take {
        inner: SimpleBuf::new(b"hello world".to_vec()),
        limit: 5,
    };
    
    let inner = buf.get_mut();
    inner.advance(5);
}

