// Answer 0

#[test]
fn test_decode_table_valid_alphabet() {
    let alphabet = Alphabet {
        symbols: [
            0u8, 1, 2, 3, 4, 5, 6, 7,
            8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39,
            40, 41, 42, 43, 44, 45, 46, 47,
            48, 49, 50, 51, 52, 53, 54, 55,
            56, 57, 58, 59, 60, 61, 62, 63,
        ],
    };
    decode_table(&alphabet);
}

#[test]
fn test_decode_table_invalid_index() {
    let alphabet = Alphabet {
        symbols: [0u8; 64],
    };
    let result = decode_table(&alphabet);
    // The output for indices 64 to 255 will be INVALID_VALUE
    let expected_value = INVALID_VALUE;
    for i in 64..256 {
        assert_eq!(result[i], expected_value);
    }
}

#[test]
fn test_decode_table_empty_alphabet() {
    let alphabet = Alphabet {


