// Answer 0

fn test_write_final_leftovers_success() {
    struct DummyEngine;

    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 3 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<(), ()> { Ok(()) }
        fn config(&self) -> &Self::Config { &() }
        
        fn encode_slice<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
            output_buf[..input.as_ref().len()].copy_from_slice(input.as_ref());
            Ok(input.as_ref().len())
        }
    }

    struct DummyWriter {
        written: Vec<u8>,
    }

    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
    }

    let engine = DummyEngine;
    let mut writer = DummyWriter { written: Vec::new() };
    let mut encoder = EncoderWriter {
        engine: &engine,
        delegate: Some(writer),
        extra_input: [1, 2, 3],
        extra_input_occupied_len: 3,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let result = encoder.write_final_leftovers();
    
    assert!(result.is_ok());
    assert!(encoder.extra_input_occupied_len == 0);
}

fn test_write_final_leftovers_write_all_encoded_failed() {
    struct DummyEngine;

    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 3 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<(), ()> { Ok(()) }
        fn config(&self) -> &Self::Config { &() }

        fn encode_slice<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
            output_buf[..input.as_ref().len()].copy_from_slice(input.as_ref());
            Ok(input.as_ref().len())
        }
    }

    struct ErroneousWriter;

    impl io::Write for ErroneousWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(ErrorKind::Interrupted, "write interrupted"))
        }

        fn flush(&mut self) -> io::Result<()> { Ok(()) }
    }

    let engine = DummyEngine;
    let mut writer = ErroneousWriter;
    let mut encoder = EncoderWriter {
        engine: &engine,
        delegate: Some(writer),
        extra_input: [1, 2, 3],
        extra_input_occupied_len: 3,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let result = encoder.write_final_leftovers();
    
    assert!(result.is_err());
}

