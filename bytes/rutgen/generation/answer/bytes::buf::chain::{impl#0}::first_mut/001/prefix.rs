// Answer 0

#[test]
fn test_first_mut_with_non_empty_buf() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data }
        }
        
        fn advance(&mut self, n: usize) {
            self.data.drain(0..n);
        }

        fn copy_to_bytes(&self) -> &[u8] {
            &self.data
        }
    }
    
    let mut a = TestBuf::new(vec![1, 2, 3]);
    let b = vec![4, 5, 6];
    let mut chain = Chain::new(a, b);

    let first = chain.first_mut();
    first.advance(1);
}

#[test]
fn test_first_mut_with_empty_buf() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data }
        }
        
        fn advance(&mut self, n: usize) {
            self.data.drain(0..n);
        }

        fn copy_to_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let mut a = TestBuf::new(vec![]);
    let b = vec![4, 5, 6];
    let mut chain = Chain::new(a, b);

    let first = chain.first_mut();
    first.advance(0);
}

#[test]
fn test_first_mut_with_large_buf() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data }
        }
        
        fn advance(&mut self, n: usize) {
            self.data.drain(0..n);
        }

        fn copy_to_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let mut a = TestBuf::new((0..1000).collect());
    let b = vec![4, 5, 6];
    let mut chain = Chain::new(a, b);

    let first = chain.first_mut();
    first.advance(500);
}

#[test]
fn test_first_mut_with_edge_case() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data }
        }
        
        fn advance(&mut self, n: usize) {
            self.data.drain(0..n);
        }

        fn copy_to_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let mut a = TestBuf::new(vec![255, 255, 255]);
    let b = vec![4, 5, 6];
    let mut chain = Chain::new(a, b);

    let first = chain.first_mut();
    first.advance(1);
}

