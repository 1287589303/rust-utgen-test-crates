// Answer 0

#[test]
fn test_from_bytes_unchecked_basic() {
    let slice: &[u8] = &[
        0x01, 0x00, 0x00, 0x00, // accel_len = 1
        0x01, 0x00, 0x00, 0x00, // first AccelTy
        0x02, 0x00, 0x00, 0x00, // second AccelTy (for valid offset)
        0x00, 0x00, 0x00, 0x00, // padding for alignment
        0x00, 0x00, 0x00, 0x00  // more padding
    ];
    let _ = regex_automata::dfa::accel::from_bytes_unchecked(slice);
}

#[test]
fn test_from_bytes_unchecked_multiple_accel() {
    let slice: &[u8] = &[
        0x02, 0x00, 0x00, 0x00, // accel_len = 2
        0x01, 0x00, 0x00, 0x00, // first AccelTy
        0x02, 0x00, 0x00, 0x00, // second AccelTy
        0x00, 0x00, 0x00, 0x00, // padding for alignment
        0x00, 0x00, 0x00, 0x00  // more padding
    ];
    let _ = regex_automata::dfa::accel::from_bytes_unchecked(slice);
}

#[test]
fn test_from_bytes_unchecked_large_accel_len() {
    let slice: &[u8] = &[
        0xFF, 0xFF, 0xFF, 0x7F, // accel_len = 2147483647
    ];
    let slice = {
        let accel_tys: Vec<u8> = (0..(2147483647 * 4))
            .map(|_| 0x00)
            .collect();
        [slice, &accel_tys[..]].concat()
    };
    let _ = regex_automata::dfa::accel::from_bytes_unchecked(&slice);
}

#[test]
fn test_from_bytes_unchecked_alignment_check() {
    let slice: &[u8] = &[
        0x01, 0x00, 0x00, 0x00, // accel_len = 1
        0x01, 0x00, 0x00, 0x00, // first AccelTy
        0x00, 0x00, 0x00, 0x00, // padding for alignment
    ];
    let _ = regex_automata::dfa::accel::from_bytes_unchecked(slice);
}

#[test]
fn test_from_bytes_unchecked_minimum_slice() {
    let slice: &[u8] = &[
        0x01, 0x00, 0x00, 0x00, // accel_len = 1
        0x01, 0x00, 0x00, 0x00, // first AccelTy
    ];
    let _ = regex_automata::dfa::accel::from_bytes_unchecked(slice);
}

