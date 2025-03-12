// Answer 0

#[test]
fn test_alphabet_len_empty() {
    let byte_classes = ByteClasses([0; 256]);
    let result = byte_classes.alphabet_len();
}

#[test]
fn test_alphabet_len_single_class() {
    let mut byte_classes = ByteClasses([0; 255]);
    byte_classes.0[255] = 0;
    let result = byte_classes.alphabet_len();
}

#[test]
fn test_alphabet_len_multiple_classes() {
    let mut byte_classes = ByteClasses([0; 256]);
    byte_classes.0[255] = 2;
    let result = byte_classes.alphabet_len();
}

#[test]
fn test_alphabet_len_maximum_classes() {
    let mut byte_classes = ByteClasses([0; 256]);
    byte_classes.0[255] = 255;
    let result = byte_classes.alphabet_len();
}

