fn from_bytes_unchecked(bytes: [u8; 4]) -> Accel {
        Accel { bytes: [bytes[0], bytes[1], bytes[2], bytes[3], 0, 0, 0, 0] }
    }