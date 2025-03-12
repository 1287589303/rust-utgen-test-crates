pub fn new(start: char, end: char) -> Self {
        let range =
            ScalarRange { start: u32::from(start), end: u32::from(end) };
        Utf8Sequences { range_stack: vec![range] }
    }