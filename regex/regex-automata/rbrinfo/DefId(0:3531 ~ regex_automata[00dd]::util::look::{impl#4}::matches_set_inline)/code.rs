pub(crate) fn matches_set_inline(
        &self,
        set: LookSet,
        haystack: &[u8],
        at: usize,
    ) -> bool {
        // This used to luse LookSet::iter with Look::matches on each element,
        // but that proved to be quite diastrous for perf. The manual "if
        // the set has this assertion, check it" turns out to be quite a bit
        // faster.
        if set.contains(Look::Start) {
            if !self.is_start(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::End) {
            if !self.is_end(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::StartLF) {
            if !self.is_start_lf(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::EndLF) {
            if !self.is_end_lf(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::StartCRLF) {
            if !self.is_start_crlf(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::EndCRLF) {
            if !self.is_end_crlf(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::WordAscii) {
            if !self.is_word_ascii(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::WordAsciiNegate) {
            if !self.is_word_ascii_negate(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::WordUnicode) {
            if !self.is_word_unicode(haystack, at).unwrap() {
                return false;
            }
        }
        if set.contains(Look::WordUnicodeNegate) {
            if !self.is_word_unicode_negate(haystack, at).unwrap() {
                return false;
            }
        }
        if set.contains(Look::WordStartAscii) {
            if !self.is_word_start_ascii(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::WordEndAscii) {
            if !self.is_word_end_ascii(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::WordStartUnicode) {
            if !self.is_word_start_unicode(haystack, at).unwrap() {
                return false;
            }
        }
        if set.contains(Look::WordEndUnicode) {
            if !self.is_word_end_unicode(haystack, at).unwrap() {
                return false;
            }
        }
        if set.contains(Look::WordStartHalfAscii) {
            if !self.is_word_start_half_ascii(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::WordEndHalfAscii) {
            if !self.is_word_end_half_ascii(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::WordStartHalfUnicode) {
            if !self.is_word_start_half_unicode(haystack, at).unwrap() {
                return false;
            }
        }
        if set.contains(Look::WordEndHalfUnicode) {
            if !self.is_word_end_half_unicode(haystack, at).unwrap() {
                return false;
            }
        }
        true
    }