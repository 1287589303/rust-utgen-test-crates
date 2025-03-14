// Answer 0

#[test]
fn test_drop_when_panicked() {
    struct Encoder {
        panicked: bool,
    }

    impl Encoder {
        fn drop(&mut self) {
            if !self.panicked {
                let _ = self.write_final_leftovers();
            }
        }

        fn write_final_leftovers(&self) -> Result<(), ()> {
            // Simulate writing leftovers
            Ok(())
        }
    }

    let mut encoder = Encoder { panicked: true };
    encoder.drop(); // Should not panic or write leftovers
}

#[test]
fn test_drop_when_not_panicked() {
    struct Encoder {
        panicked: bool,
    }

    impl Encoder {
        fn drop(&mut self) {
            if !self.panicked {
                let _ = self.write_final_leftovers();
            }
        }

        fn write_final_leftovers(&self) -> Result<(), ()> {
            // Simulate writing leftovers
            Ok(())
        }
    }

    let mut encoder = Encoder { panicked: false };
    encoder.drop(); // Should not panic
}

