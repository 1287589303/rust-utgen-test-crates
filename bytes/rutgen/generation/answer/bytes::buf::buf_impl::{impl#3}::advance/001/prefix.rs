// Answer 0

#[test]
fn test_advance_overflow() {
    struct Cursor {
        position: u64,
        data: Vec<u8>,
    }

    impl Cursor {
        fn new(data: Vec<u8>) -> Self {
            Self { position: 0, data }
        }
        
        fn get_ref(&self) -> &Vec<u8> {
            &self.data
        }

        fn position(&self) -> u64 {
            self.position
        }

        fn set_position(&mut self, pos: u64) {
            self.position = pos;
        }
    }

    impl std::io::Cursor<Vec<u8>> for Cursor {
        fn remaining(&self) -> usize {
            self.get_ref().len() - self.position as usize
        }

        fn advance(&mut self, cnt: usize) {
            let len = self.get_ref().len();
            let pos = self.position();
            let max_cnt = saturating_sub_usize_u64(len, pos);
            if cnt > max_cnt {
                panic_advance(&TryGetError {
                    requested: cnt,
                    available: max_cnt,
                });
            }
            self.set_position(pos + cnt as u64);
        }
    }

    let data = vec![0u8; 10];
    let mut cursor = Cursor::new(data);
    cursor.set_position(8); // Set position near the end
    let cnt = 3; // Allows us to exceed the maximum count

    // This call should trigger panic due to overflow
    cursor.advance(cnt);
}

