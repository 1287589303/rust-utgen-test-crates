// Answer 0

#[test]
fn test_invalid_base64_unexpected_symbol() {
    let error_detail = InvalidBase64Details::UnexpectedSymbol(b'!');
    let decode_error: DecodeError<()> = error_detail.into();
}

#[test]
fn test_invalid_base64_alphabet_symbol_after_padding() {
    let error_detail = InvalidBase64Details::AlphabetSymbolAfterPadding;
    let decode_error: DecodeError<()> = error_detail.into();
}

#[test]
fn test_invalid_base64_lone_alphabet_symbol() {
    let error_detail = InvalidBase64Details::LoneAlphabetSymbol;
    let decode_error: DecodeError<()> = error_detail.into();
}

#[test]
fn test_invalid_base64_padding() {
    let error_detail = InvalidBase64Details::Padding;
    let decode_error: DecodeError<()> = error_detail.into();
}

