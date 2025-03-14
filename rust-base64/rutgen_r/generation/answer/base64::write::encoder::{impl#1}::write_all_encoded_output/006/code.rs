// Answer 0

#[derive(Default)]
struct Encoder {
    output_occupied_len: usize,
}

impl Encoder {
    fn write_to_delegate(&mut self, _len: usize) -> Result<(), std::io::Error> {
        // Simulate a successful write
        Ok(())
    }

    fn write_all_encoded_output(&mut self) -> Result<()> {
        while self.output_occupied_len > 0 {
            let remaining_len = self.output_occupied_len;
            match self.write_to_delegate(remaining_len) {
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
                Ok(()) => {}
            }
        }

        debug_assert_eq!(0, self.output_occupied_len);
        Ok(())
    }
}

#[test]
fn test_write_all_encoded_output_no_data() {
    let mut encoder = Encoder::default();
    encoder.output_occupied_len = 0;

    let result = encoder.write_all_encoded_output();
    assert!(result.is_ok());
}

