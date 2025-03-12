fn fast_u16_to_str(
    // max 5 digits for u16 (65535)
    buffer: &mut [u8; 5],
    mut value: u16,
) -> &str {
    let mut index = buffer.len();

    loop {
        index -= 1;
        buffer[index] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }

    // SAFETY: we know the values in the buffer from the
    // current index on will be a number
    unsafe { core::str::from_utf8_unchecked(&buffer[index..]) }
}