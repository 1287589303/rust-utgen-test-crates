// Answer 0

#[test]
fn test_drop_when_not_panicked() {
    struct TestEncoder {
        panicked: bool,
    }

    impl TestEncoder {
        fn new() -> Self {
            Self {
                panicked: false,
            }
        }

        fn drop(&mut self) {
            if !self.panicked {
                // like `BufWriter`, ignore errors during drop
                let _ = self.write_final_leftovers();
            }
        }

        fn write_final_leftovers(&self) -> Result<(), ()> {
            // Simulated behavior for writing leftovers
            Ok(())
        }
    }

    let mut encoder = TestEncoder::new();
    encoder.drop();  // The drop method is tested here
}

#[test]
fn test_drop_multiple_calls_when_not_panicked() {
    struct TestEncoder {
        panicked: bool,
    }

    impl TestEncoder {
        fn new() -> Self {
            Self {
                panicked: false,
            }
        }

        fn drop(&mut self) {
            if !self.panicked {
                // like `BufWriter`, ignore errors during drop
                let _ = self.write_final_leftovers();
            }
        }

        fn write_final_leftovers(&self) -> Result<(), ()> {
            // Simulated behavior for writing leftovers
            Ok(())
        }
    }

    let mut encoder = TestEncoder::new();
    encoder.drop();  // First call
    encoder.drop();  // Second call, should not panic or fail
}

