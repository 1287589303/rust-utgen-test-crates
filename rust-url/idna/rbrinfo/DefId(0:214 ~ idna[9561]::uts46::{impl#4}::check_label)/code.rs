fn check_label(
        &self,
        hyphens: Hyphens,
        mut_label: &mut [char],
        fail_fast: bool,
        had_errors: &mut bool,
        first_needs_combining_mark_check: bool,
        needs_contextj_check: bool,
    ) -> bool {
        if hyphens != Hyphens::Allow
            && check_hyphens(
                mut_label,
                hyphens == Hyphens::CheckFirstLast,
                fail_fast,
                had_errors,
            )
        {
            return true;
        }
        if first_needs_combining_mark_check {
            if let Some(first) = mut_label.first_mut() {
                if self.data.is_mark(*first) {
                    if fail_fast {
                        return true;
                    }
                    *had_errors = true;
                    *first = '\u{FFFD}';
                }
            }
        }
        if needs_contextj_check {
            // ContextJ
            for i in 0..mut_label.len() {
                let c = mut_label[i];
                if !in_inclusive_range_char(c, '\u{200C}', '\u{200D}') {
                    continue;
                }
                let (head, joiner_and_tail) = mut_label.split_at_mut(i);

                if let Some((joiner, tail)) = joiner_and_tail.split_first_mut() {
                    if let Some(previous) = head.last() {
                        if self.data.is_virama(*previous) {
                            continue;
                        }
                    } else {
                        // No preceding character
                        if fail_fast {
                            return true;
                        }
                        *had_errors = true;
                        *joiner = '\u{FFFD}';
                        continue;
                    }
                    if c == '\u{200D}' {
                        // ZWJ only has the virama rule
                        if fail_fast {
                            return true;
                        }
                        *had_errors = true;
                        *joiner = '\u{FFFD}';
                        continue;
                    }
                    debug_assert_eq!(c, '\u{200C}');
                    if !self.has_appropriately_joining_char(
                        head.iter().rev().copied(),
                        LEFT_OR_DUAL_JOINING_MASK,
                    ) || !self.has_appropriately_joining_char(
                        tail.iter().copied(),
                        RIGHT_OR_DUAL_JOINING_MASK,
                    ) {
                        if fail_fast {
                            return true;
                        }
                        *had_errors = true;
                        *joiner = '\u{FFFD}';
                    }
                } else {
                    debug_assert!(false);
                }
            }
        }

        if !is_ascii(mut_label) && mut_label.len() > PUNYCODE_ENCODE_MAX_INPUT_LENGTH {
            // Limit quadratic behavior
            // https://github.com/whatwg/url/issues/824
            // https://unicode-org.atlassian.net/browse/ICU-13727
            if fail_fast {
                return true;
            }
            *had_errors = true;
            mut_label[PUNYCODE_ENCODE_MAX_INPUT_LENGTH] = '\u{FFFD}';
        }
        false
    }