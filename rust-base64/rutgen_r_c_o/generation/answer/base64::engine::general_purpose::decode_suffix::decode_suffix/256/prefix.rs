// Answer 0

#[test]
fn test_decode_suffix_invalid_padding() {
    let input: &[u8] = &[0_u8, 1_u8, 2_u8, 3_u8, 61_u8]; // 61 is '=' in base64
    let input_index: usize = 0;
    let mut output: [u8; 10] = [0; 10];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x41] = 0; // A
        table[0x42] = 1; // B
        table[0x43] = 2; // C
        table[0x44] = 3; // D
        table[0x45] = 4; // E
        table[0x46] = 5; // F
        table[0x47] = 6; // G
        table[0x48] = 7; // H
        table[0x49] = 8; // I
        table[0x4A] = 9; // J
        table[0x4B] = 10; // K
        table[0x4C] = 11; // L
        table[0x4D] = 12; // M
        table[0x4E] = 13; // N
        table[0x4F] = 14; // O
        table[0x50] = 15; // P
        table[0x51] = 16; // Q
        table[0x52] = 17; // R
        table[0x53] = 18; // S
        table[0x54] = 19; // T
        table[0x55] = 20; // U
        table[0x56] = 21; // V
        table[0x57] = 22; // W
        table[0x58] = 23; // X
        table[0x59] = 24; // Y
        table[0x5A] = 25; // Z
        table[0x61] = 26; // a
        table[0x62] = 27; // b
        table[0x63] = 28; // c
        table[0x64] = 29; // d
        table[0x65] = 30; // e
        table[0x66] = 31; // f
        table[0x67] = 32; // g
        table[0x68] = 33; // h
        table[0x69] = 34; // i
        table[0x6A] = 35; // j
        table[0x6B] = 36; // k
        table[0x6C] = 37; // l
        table[0x6D] = 38; // m
        table[0x6E] = 39; // n
        table[0x6F] = 40; // o
        table[0x70] = 41; // p
        table[0x71] = 42; // q
        table[0x72] = 43; // r
        table[0x73] = 44; // s
        table[0x74] = 45; // t
        table[0x75] = 46; // u
        table[0x76] = 47; // v
        table[0x77] = 48; // w
        table[0x78] = 49; // x
        table[0x79] = 50; // y
        table[0x7A] = 51; // z
        table[0x30] = 52; // 0
        table[0x31] = 53; // 1
        table[0x32] = 54; // 2
        table[0x33] = 55; // 3
        table[0x34] = 56; // 4
        table[0x35] = 57; // 5
        table[0x36] = 58; // 6
        table[0x37] = 59; // 7
        table[0x38] = 60; // 8
        table[0x39] = 61; // 9
        table[0x2B] = 62; // +
        table[0x2F] = 63; // /
        table
    };
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::RequireNone;

    let _ = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );
}

