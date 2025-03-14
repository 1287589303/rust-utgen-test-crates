// Answer 0

#[test]
fn test_encode_table_boundary_case() {
    let alphabet = Alphabet {
        symbols: [
            b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
            b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'0', b'1', b'2', b'3', b'4', b'5',
            b'6', b'7', b'8', b'9', b'+', b'/', b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')',
            b'-', b'_', b'=', b'+', b'{', b'}', b'[', b']', b'|', b':', b';', b'"', b'\'', b'<', b'>', b',',
            b'.', b'?', b'/', b'~'
        ],
    };
    let _result = encode_table(&alphabet);
}

#[test]
#[should_panic]
fn test_encode_table_overflow_case() {
    let alphabet = Alphabet {
        symbols: [
            b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
            b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'0', b'1', b'2', b'3', b'4', b'5',
            b'6', b'7', b'8', b'9', b'+', b'/', b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')',
            b'-', b'_', b'=', b'+', b'{', b'}', b'[', b']', b'|', b':', b';', b'"', b'\'', b'<', b'>', b',',
            b'.', b'?', b'/',
        ],
    };
    let _result = encode_table(&alphabet);
}

