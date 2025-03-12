pub fn try_is_word_character(
    c: char,
) -> core::result::Result<bool, UnicodeWordError> {
    unicode::is_word_character(c)
}