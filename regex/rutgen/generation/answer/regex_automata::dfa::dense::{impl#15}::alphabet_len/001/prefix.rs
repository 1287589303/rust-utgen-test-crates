// Answer 0

#[test]
fn test_alphabet_len_power_of_2() {
    let byte_classes = ByteClasses([0; 256]);
    let mut class = byte_classes.clone();
    
    class.0[1] = 1; // Alphabet length 2
    let tt_2 = TransitionTable { table: vec![0; 2], classes: class, stride2: 1 };
    tt_2.alphabet_len();
    
    class.0[3] = 3; // Alphabet length 4
    let tt_4 = TransitionTable { table: vec![0; 4], classes: class, stride2: 2 };
    tt_4.alphabet_len();

    class.0[7] = 7; // Alphabet length 8
    let tt_8 = TransitionTable { table: vec![0; 8], classes: class, stride2: 3 };
    tt_8.alphabet_len();
    
    class.0[15] = 15; // Alphabet length 16
    let tt_16 = TransitionTable { table: vec![0; 16], classes: class, stride2: 4 };
    tt_16.alphabet_len();
    
    class.0[31] = 31; // Alphabet length 32
    let tt_32 = TransitionTable { table: vec![0; 32], classes: class, stride2: 5 };
    tt_32.alphabet_len();
    
    class.0[63] = 63; // Alphabet length 64
    let tt_64 = TransitionTable { table: vec![0; 64], classes: class, stride2: 6 };
    tt_64.alphabet_len();

    class.0[127] = 127; // Alphabet length 128
    let tt_128 = TransitionTable { table: vec![0; 128], classes: class, stride2: 7 };
    tt_128.alphabet_len();

    class.0[255] = 255; // Alphabet length 256
    let tt_256 = TransitionTable { table: vec![0; 256], classes: class, stride2: 8 };
    tt_256.alphabet_len();
}

#[test]
fn test_alphabet_len_non_power_of_2() {
    let byte_classes = ByteClasses([0; 256]);
    let mut class = byte_classes.clone();

    // Testing with various non-power of 2 values.
    
    class.0[2] = 2; // Alphabet length 3
    let tt_3 = TransitionTable { table: vec![0; 3], classes: class, stride2: 2 };
    tt_3.alphabet_len();
    
    class.0[4] = 4; // Alphabet length 5
    let tt_5 = TransitionTable { table: vec![0; 5], classes: class, stride2: 3 };
    tt_5.alphabet_len();

    class.0[6] = 6; // Alphabet length 7
    let tt_7 = TransitionTable { table: vec![0; 7], classes: class, stride2: 3 };
    tt_7.alphabet_len();
    
    class.0[8] = 8; // Alphabet length 9
    let tt_9 = TransitionTable { table: vec![0; 9], classes: class, stride2: 4 };
    tt_9.alphabet_len();
    
    class.0[9] = 9; // Alphabet length 10
    let tt_10 = TransitionTable { table: vec![0; 10], classes: class, stride2: 4 };
    tt_10.alphabet_len();

    class.0[10] = 10; // Alphabet length 11
    let tt_11 = TransitionTable { table: vec![0; 11], classes: class, stride2: 4 };
    tt_11.alphabet_len();
    
    // ... and so on up to the max values before 256
    for alphabet_size in 12..=255 {
        class.0[alphabet_size - 1] = alphabet_size as u8;
        let tt = TransitionTable { table: vec![0; alphabet_size], classes: class, stride2: 8 };
        tt.alphabet_len();
    }
}

