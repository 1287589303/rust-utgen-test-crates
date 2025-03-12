use alloc::{boxed::Box, string::String, vec, vec::Vec};
use crate::{error::Error, utf8};
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Look {
    /// Match the beginning of text. Specifically, this matches at the starting
    /// position of the input.
    Start = 1 << 0,
    /// Match the end of text. Specifically, this matches at the ending
    /// position of the input.
    End = 1 << 1,
    /// Match the beginning of a line or the beginning of text. Specifically,
    /// this matches at the starting position of the input, or at the position
    /// immediately following a `\n` character.
    StartLF = 1 << 2,
    /// Match the end of a line or the end of text. Specifically, this matches
    /// at the end position of the input, or at the position immediately
    /// preceding a `\n` character.
    EndLF = 1 << 3,
    /// Match the beginning of a line or the beginning of text. Specifically,
    /// this matches at the starting position of the input, or at the position
    /// immediately following either a `\r` or `\n` character, but never after
    /// a `\r` when a `\n` follows.
    StartCRLF = 1 << 4,
    /// Match the end of a line or the end of text. Specifically, this matches
    /// at the end position of the input, or at the position immediately
    /// preceding a `\r` or `\n` character, but never before a `\n` when a `\r`
    /// precedes it.
    EndCRLF = 1 << 5,
    /// Match an ASCII-only word boundary. That is, this matches a position
    /// where the left adjacent character and right adjacent character
    /// correspond to a word and non-word or a non-word and word character.
    Word = 1 << 6,
    /// Match an ASCII-only negation of a word boundary.
    WordNegate = 1 << 7,
    /// Match the start of an ASCII-only word boundary. That is, this matches a
    /// position at either the beginning of the haystack or where the previous
    /// character is not a word character and the following character is a word
    /// character.
    WordStart = 1 << 8,
    /// Match the end of an ASCII-only word boundary. That is, this matches
    /// a position at either the end of the haystack or where the previous
    /// character is a word character and the following character is not a word
    /// character.
    WordEnd = 1 << 9,
    /// Match the start half of an ASCII-only word boundary. That is, this
    /// matches a position at either the beginning of the haystack or where the
    /// previous character is not a word character.
    WordStartHalf = 1 << 10,
    /// Match the end half of an ASCII-only word boundary. That is, this
    /// matches a position at either the end of the haystack or where the
    /// following character is not a word character.
    WordEndHalf = 1 << 11,
}
impl Look {
    pub(crate) fn is_match(&self, haystack: &[u8], at: usize) -> bool {
        use self::Look::*;
        match *self {
            Start => at == 0,
            End => at == haystack.len(),
            StartLF => at == 0 || haystack[at - 1] == b'\n',
            EndLF => at == haystack.len() || haystack[at] == b'\n',
            StartCRLF => {
                at == 0 || haystack[at - 1] == b'\n'
                    || (haystack[at - 1] == b'\r'
                        && (at >= haystack.len() || haystack[at] != b'\n'))
            }
            EndCRLF => {
                at == haystack.len() || haystack[at] == b'\r'
                    || (haystack[at] == b'\n' && (at == 0 || haystack[at - 1] != b'\r'))
            }
            Word => {
                let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                word_before != word_after
            }
            WordNegate => {
                let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                word_before == word_after
            }
            WordStart => {
                let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                !word_before && word_after
            }
            WordEnd => {
                let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                word_before && !word_after
            }
            WordStartHalf => {
                let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                !word_before
            }
            WordEndHalf => {
                let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                !word_after
            }
        }
    }
}
pub(crate) fn is_word_byte(b: u8) -> bool {
    const fn mkwordset() -> [bool; 256] {
        let mut set = [false; 256];
        set[b'_' as usize] = true;
        let mut byte = b'0';
        while byte <= b'9' {
            set[byte as usize] = true;
            byte += 1;
        }
        byte = b'A';
        while byte <= b'Z' {
            set[byte as usize] = true;
            byte += 1;
        }
        byte = b'a';
        while byte <= b'z' {
            set[byte as usize] = true;
            byte += 1;
        }
        set
    }
    const WORD: [bool; 256] = mkwordset();
    WORD[b as usize]
}
