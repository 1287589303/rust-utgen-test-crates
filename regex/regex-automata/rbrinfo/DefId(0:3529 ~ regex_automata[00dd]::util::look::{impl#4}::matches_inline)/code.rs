pub(crate) fn matches_inline(
        &self,
        look: Look,
        haystack: &[u8],
        at: usize,
    ) -> bool {
        match look {
            Look::Start => self.is_start(haystack, at),
            Look::End => self.is_end(haystack, at),
            Look::StartLF => self.is_start_lf(haystack, at),
            Look::EndLF => self.is_end_lf(haystack, at),
            Look::StartCRLF => self.is_start_crlf(haystack, at),
            Look::EndCRLF => self.is_end_crlf(haystack, at),
            Look::WordAscii => self.is_word_ascii(haystack, at),
            Look::WordAsciiNegate => self.is_word_ascii_negate(haystack, at),
            Look::WordUnicode => self.is_word_unicode(haystack, at).unwrap(),
            Look::WordUnicodeNegate => {
                self.is_word_unicode_negate(haystack, at).unwrap()
            }
            Look::WordStartAscii => self.is_word_start_ascii(haystack, at),
            Look::WordEndAscii => self.is_word_end_ascii(haystack, at),
            Look::WordStartUnicode => {
                self.is_word_start_unicode(haystack, at).unwrap()
            }
            Look::WordEndUnicode => {
                self.is_word_end_unicode(haystack, at).unwrap()
            }
            Look::WordStartHalfAscii => {
                self.is_word_start_half_ascii(haystack, at)
            }
            Look::WordEndHalfAscii => {
                self.is_word_end_half_ascii(haystack, at)
            }
            Look::WordStartHalfUnicode => {
                self.is_word_start_half_unicode(haystack, at).unwrap()
            }
            Look::WordEndHalfUnicode => {
                self.is_word_end_half_unicode(haystack, at).unwrap()
            }
        }
    }