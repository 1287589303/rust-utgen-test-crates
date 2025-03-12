fn decode_step(state: &mut usize, cp: &mut u32, b: u8) {
    // Splits the space of all bytes into equivalence classes, such that
    // any byte in the same class can never discriminate between whether a
    // particular sequence is valid UTF-8 or not.
    #[rustfmt::skip]
    const CLASSES: [u8; 256] = [
       0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
       0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
       0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
       0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
       1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,  9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,
       7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,  7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,
       8,8,2,2,2,2,2,2,2,2,2,2,2,2,2,2,  2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
      10,3,3,3,3,3,3,3,3,3,3,3,3,4,3,3, 11,6,6,6,5,8,8,8,8,8,8,8,8,8,8,8,
    ];

    // A state machine taken from `bstr` which was in turn adapted from:
    // https://bjoern.hoehrmann.de/utf-8/decoder/dfa/
    #[rustfmt::skip]
    const STATES_FORWARD: &'static [u8] = &[
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      12, 0, 24, 36, 60, 96, 84, 0, 0, 0, 48, 72,
      0, 12, 0, 0, 0, 0, 0, 12, 0, 12, 0, 0,
      0, 24, 0, 0, 0, 0, 0, 24, 0, 24, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0,
      0, 24, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 36, 0, 36, 0, 0,
      0, 36, 0, 0, 0, 0, 0, 36, 0, 36, 0, 0,
      0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let class = CLASSES[usize::from(b)];
    if *state == ACCEPT {
        *cp = (0xFF >> class) & (b as u32);
    } else {
        *cp = (b as u32 & 0b111111) | (*cp << 6);
    }
    *state = usize::from(STATES_FORWARD[*state + usize::from(class)]);
}