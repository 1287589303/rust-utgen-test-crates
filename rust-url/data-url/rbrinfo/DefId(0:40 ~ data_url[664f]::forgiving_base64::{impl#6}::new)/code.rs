pub fn new(write_bytes: F) -> Self {
        Self {
            write_bytes,
            bit_buffer: 0,
            buffer_bit_length: 0,
            padding_symbols: 0,
        }
    }