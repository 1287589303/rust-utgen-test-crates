// Answer 0

#[test]
fn test_advance_a_rem_equals_cnt() {
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
    }

    let mut a = TestBuf {
        data: vec![1, 2, 3, 4],
        position: 0,
    };
    let mut b = TestBuf {
        data: vec![5, 6, 7, 8],
        position: 0,
    };

    let mut chain_buf = Chain { a, b };
    
    let cnt = chain_buf.a.remaining(); // Here, a_rem == cnt and also > 0
    chain_buf.advance(cnt);
}

#[test]
fn test_advance_a_rem_greater_than_cnt() {
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
    }

    let mut a = TestBuf {
        data: vec![1, 2, 3, 4, 5],
        position: 0,
    };
    let mut b = TestBuf {
        data: vec![6, 7, 8, 9],
        position: 0,
    };

    let mut chain_buf = Chain { a, b };
    
    let cnt = 3; // a_rem > cnt (which is 3)
    chain_buf.advance(cnt);
}

