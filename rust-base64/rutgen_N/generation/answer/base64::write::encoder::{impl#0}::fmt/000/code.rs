// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct Encoder {
        extra_input: String,
        extra_input_occupied_len: usize,
        output: Vec<u8>,
        output_occupied_len: usize,
    }

    impl fmt::Display for Encoder {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "extra_input: {:?} extra_input_occupied_len:{:?} output[..5]: {:?} output_occupied_len: {:?}",
                self.extra_input,
                self.extra_input_occupied_len,
                &self.output[0..5],
                self.output_occupied_len
            )
        }
    }

    let encoder = Encoder {
        extra_input: String::from("example"),
        extra_input_occupied_len: 7,
        output: vec![1, 2, 3, 4, 5, 6],
        output_occupied_len: 6,
    };

    let result = format!("{}", encoder);
    assert!(result.contains("extra_input: \"example\""));
    assert!(result.contains("extra_input_occupied_len: 7"));
    assert!(result.contains("output[..5]: [1, 2, 3, 4, 5]"));
    assert!(result.contains("output_occupied_len: 6"));
}

#[test]
fn test_fmt_empty_output() {
    use std::fmt;

    struct Encoder {
        extra_input: String,
        extra_input_occupied_len: usize,
        output: Vec<u8>,
        output_occupied_len: usize,
    }

    impl fmt::Display for Encoder {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "extra_input: {:?} extra_input_occupied_len:{:?} output[..5]: {:?} output_occupied_len: {:?}",
                self.extra_input,
                self.extra_input_occupied_len,
                &self.output[0..5],
                self.output_occupied_len
            )
        }
    }

    let encoder = Encoder {
        extra_input: String::from(""),
        extra_input_occupied_len: 0,
        output: vec![],
        output_occupied_len: 0,
    };

    let result = format!("{}", encoder);
    assert!(result.contains("extra_input: \"\""));
    assert!(result.contains("extra_input_occupied_len: 0"));
    assert!(result.contains("output[..5]: []"));
    assert!(result.contains("output_occupied_len: 0"));
}

