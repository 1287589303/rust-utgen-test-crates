// Answer 0

#[derive(Default)]
struct Encoder {
    panicked: bool,
}

impl Encoder {
    fn write_final_leftovers(&mut self) -> Result<(), &'static str> {
        // Simulating final leftovers writing
        Ok(())
    }
    
    fn drop(&mut self) {
        if !self.panicked {
            // like `BufWriter`, ignore errors during drop
            let _ = self.write_final_leftovers();
        }
    }
}

#[test]
fn test_drop_without_panicked() {
    let mut encoder = Encoder::default();
    encoder.panicked = false;
    encoder.drop();
    // No assertion needed; if no panic occurs, the test passes
}

#[test]
#[should_panic]
fn test_drop_with_panicked() {
    let mut encoder = Encoder::default();
    encoder.panicked = true;
    encoder.drop(); // This should not panic but will test drop with panicked state
    // No assertion needed; if no panic occurs, the test passes
}

