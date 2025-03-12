fn process_innermost<'a>(
        &self,
        domain_name: &'a [u8],
        ascii_deny_list: AsciiDenyList,
        hyphens: Hyphens,
        fail_fast: bool,
        domain_buffer: &mut SmallVec<[char; 253]>,
        already_punycode: &mut SmallVec<[AlreadyAsciiLabel<'a>; 8]>,
        tail: &'a [u8],
    ) -> (usize, bool, bool) {
        let deny_list = ascii_deny_list.bits;
        let deny_list_deny_dot = deny_list | DOT_MASK;

        let mut had_errors = false;

        let mut passthrough_up_to = domain_name.len() - tail.len(); // Index into `domain_name`
                                                                    // 253 ASCII characters is the max length for a valid domain name
                                                                    // (excluding the root dot).
        let mut current_label_start; // Index into `domain_buffer`
        let mut seen_label = false;
        let mut in_prefix = true;
        for label in tail.split(|b| *b == b'.') {
            // We check for passthrough only for the prefix. That is, if we
            // haven't moved on and started filling `domain_buffer`. Keeping
            // this stuff in one loop where the first items keep being skipped
            // once they have been skipped at least once instead of working
            // this into a fancier loop structure in order to make sure that
            // no item from the iterator is lost or processed twice.
            // Furthermore, after the passthrough fails, restarting the
            // normalization process after each pre-existing ASCII dot also
            // provides an opportunity for the processing to get back onto
            // an ASCII fast path that bypasses the normalizer for ASCII
            // after a pre-existing ASCII dot (pre-existing in the sense
            // of not coming from e.g. normalizing an ideographic dot).
            if in_prefix && is_passthrough_ascii_label(label) {
                if seen_label {
                    debug_assert_eq!(domain_name[passthrough_up_to], b'.');
                    passthrough_up_to += 1;
                }
                seen_label = true;

                passthrough_up_to += label.len();
                continue;
            }
            if seen_label {
                if in_prefix {
                    debug_assert_eq!(domain_name[passthrough_up_to], b'.');
                    passthrough_up_to += 1;
                } else {
                    domain_buffer.push('.');
                }
            }
            seen_label = true;
            in_prefix = false;
            current_label_start = domain_buffer.len();
            if !label.is_empty() {
                let (ascii, non_ascii) = split_ascii_fast_path_prefix(label);
                let non_punycode_ascii_label = if non_ascii.is_empty() {
                    if has_punycode_prefix(ascii) {
                        if (ascii.last() != Some(&b'-'))
                            && (ascii.len() - 4 <= PUNYCODE_DECODE_MAX_INPUT_LENGTH)
                        {
                            if let Ok(decode) =
                                Decoder::default().decode::<u8, InternalCaller>(&ascii[4..])
                            {
                                // 63 ASCII characters is the max length for a valid DNS label and xn-- takes 4
                                // characters.
                                let mut label_buffer = SmallVec::<[char; 59]>::new();
                                label_buffer.extend(decode);

                                if self.after_punycode_decode(
                                    domain_buffer,
                                    current_label_start,
                                    &label_buffer,
                                    deny_list_deny_dot,
                                    fail_fast,
                                    &mut had_errors,
                                ) {
                                    return (0, false, true);
                                }

                                if self.check_label(
                                    hyphens,
                                    &mut domain_buffer[current_label_start..],
                                    fail_fast,
                                    &mut had_errors,
                                    true,
                                    true,
                                ) {
                                    return (0, false, true);
                                }
                            } else {
                                // Punycode failed
                                if fail_fast {
                                    return (0, false, true);
                                }
                                had_errors = true;
                                domain_buffer.push('\u{FFFD}');
                                let mut iter = ascii.iter();
                                // Discard the first character that we replaced.
                                let _ = iter.next();
                                domain_buffer.extend(iter.map(|c| {
                                    // Can't have dot here, so `deny_list` vs `deny_list_deny_dot` does
                                    // not matter.
                                    apply_ascii_deny_list_to_potentially_upper_case_ascii(
                                        *c, deny_list,
                                    )
                                }));
                            };
                            // If there were errors, we won't be trying to use this
                            // anyway later, so it's fine to put it here unconditionally.
                            already_punycode.push(AlreadyAsciiLabel::MixedCasePunycode(label));
                            continue;
                        } else if fail_fast {
                            return (0, false, true);
                        }
                        // Else fall through to the complex path and rediscover error
                        // there.
                        false
                    } else {
                        true
                    }
                } else {
                    false
                };
                for c in ascii.iter().map(|c| {
                    // Can't have dot here, so `deny_list` vs `deny_list_deny_dot` does
                    // not matter.
                    apply_ascii_deny_list_to_potentially_upper_case_ascii(*c, deny_list)
                }) {
                    if c == '\u{FFFD}' {
                        if fail_fast {
                            return (0, false, true);
                        }
                        had_errors = true;
                    }
                    domain_buffer.push(c);
                }
                if non_punycode_ascii_label {
                    if hyphens != Hyphens::Allow
                        && check_hyphens(
                            &mut domain_buffer[current_label_start..],
                            hyphens == Hyphens::CheckFirstLast,
                            fail_fast,
                            &mut had_errors,
                        )
                    {
                        return (0, false, true);
                    }
                    already_punycode.push(if had_errors {
                        AlreadyAsciiLabel::Other
                    } else {
                        AlreadyAsciiLabel::MixedCaseAscii(label)
                    });
                    continue;
                }
                already_punycode.push(AlreadyAsciiLabel::Other);
                let mut first_needs_combining_mark_check = ascii.is_empty();
                let mut needs_contextj_check = !non_ascii.is_empty();
                let mut mapping = self
                    .data
                    .map_normalize(non_ascii.chars())
                    .map(|c| apply_ascii_deny_list_to_lower_cased_unicode(c, deny_list));
                loop {
                    let n = mapping.next();
                    match n {
                        None | Some('.') => {
                            if domain_buffer[current_label_start..]
                                .starts_with(&['x', 'n', '-', '-'])
                            {
                                let mut punycode_precondition_failed = false;
                                for c in domain_buffer[current_label_start + 4..].iter_mut() {
                                    if !c.is_ascii() {
                                        if fail_fast {
                                            return (0, false, true);
                                        }
                                        had_errors = true;
                                        *c = '\u{FFFD}';
                                        punycode_precondition_failed = true;
                                    }
                                }

                                if let Some(last) = domain_buffer.last_mut() {
                                    if *last == '-' {
                                        // Either there's nothing after the "xn--" prefix
                                        // and we got the last hyphen of "xn--", or there
                                        // are no Punycode digits after the last delimiter
                                        // which would result in Punycode decode outputting
                                        // ASCII only.
                                        if fail_fast {
                                            return (0, false, true);
                                        }
                                        had_errors = true;
                                        *last = '\u{FFFD}';
                                        punycode_precondition_failed = true;
                                    }
                                } else {
                                    unreachable!();
                                }

                                // Reject excessively long input
                                // https://github.com/whatwg/url/issues/824
                                // https://unicode-org.atlassian.net/browse/ICU-13727
                                if domain_buffer.len() - current_label_start - 4
                                    > PUNYCODE_DECODE_MAX_INPUT_LENGTH
                                {
                                    if fail_fast {
                                        return (0, false, true);
                                    }
                                    had_errors = true;
                                    domain_buffer[current_label_start
                                        + 4
                                        + PUNYCODE_DECODE_MAX_INPUT_LENGTH] = '\u{FFFD}';
                                    punycode_precondition_failed = true;
                                }

                                if !punycode_precondition_failed {
                                    if let Ok(decode) = Decoder::default()
                                        .decode::<char, InternalCaller>(
                                            &domain_buffer[current_label_start + 4..],
                                        )
                                    {
                                        first_needs_combining_mark_check = true;
                                        needs_contextj_check = true;
                                        // 63 ASCII characters is the max length for a valid DNS label and xn-- takes 4
                                        // characters.
                                        let mut label_buffer = SmallVec::<[char; 59]>::new();
                                        label_buffer.extend(decode);

                                        domain_buffer.truncate(current_label_start);
                                        if self.after_punycode_decode(
                                            domain_buffer,
                                            current_label_start,
                                            &label_buffer,
                                            deny_list_deny_dot,
                                            fail_fast,
                                            &mut had_errors,
                                        ) {
                                            return (0, false, true);
                                        }
                                    } else {
                                        // Punycode failed
                                        if fail_fast {
                                            return (0, false, true);
                                        }
                                        had_errors = true;
                                        domain_buffer[current_label_start] = '\u{FFFD}';
                                        needs_contextj_check = false; // ASCII label
                                        first_needs_combining_mark_check = false;
                                    };
                                } else {
                                    first_needs_combining_mark_check = false;
                                    needs_contextj_check = false; // Non-ASCII already turned to U+FFFD.
                                }
                            }
                            if self.check_label(
                                hyphens,
                                &mut domain_buffer[current_label_start..],
                                fail_fast,
                                &mut had_errors,
                                first_needs_combining_mark_check,
                                needs_contextj_check,
                            ) {
                                return (0, false, true);
                            }

                            if n.is_none() {
                                break;
                            }
                            domain_buffer.push('.');
                            current_label_start = domain_buffer.len();
                            first_needs_combining_mark_check = true;
                            needs_contextj_check = true;
                            already_punycode.push(AlreadyAsciiLabel::Other);
                        }
                        Some(c) => {
                            if c == '\u{FFFD}' {
                                if fail_fast {
                                    return (0, false, true);
                                }
                                had_errors = true;
                            }
                            domain_buffer.push(c);
                        }
                    }
                }
            } else {
                // Empty label
                already_punycode.push(AlreadyAsciiLabel::MixedCaseAscii(label));
            }
        }

        let is_bidi = self.is_bidi(domain_buffer);
        if is_bidi {
            for label in domain_buffer.split_mut(|c| *c == '.') {
                if let Some((first, tail)) = label.split_first_mut() {
                    let first_bc = self.data.bidi_class(*first);
                    if !FIRST_BC_MASK.intersects(first_bc.to_mask()) {
                        // Neither RTL label nor LTR label
                        if fail_fast {
                            return (0, false, true);
                        }
                        had_errors = true;
                        *first = '\u{FFFD}';
                        continue;
                    }
                    let is_ltr = first_bc.is_ltr();
                    // Trim NSM
                    let mut middle = tail;
                    #[allow(clippy::while_let_loop)]
                    loop {
                        if let Some((last, prior)) = middle.split_last_mut() {
                            let last_bc = self.data.bidi_class(*last);
                            if last_bc.is_nonspacing_mark() {
                                middle = prior;
                                continue;
                            }
                            let last_mask = if is_ltr { LAST_LTR_MASK } else { LAST_RTL_MASK };
                            if !last_mask.intersects(last_bc.to_mask()) {
                                if fail_fast {
                                    return (0, false, true);
                                }
                                had_errors = true;
                                *last = '\u{FFFD}';
                            }
                            if is_ltr {
                                for c in prior.iter_mut() {
                                    let bc = self.data.bidi_class(*c);
                                    if !MIDDLE_LTR_MASK.intersects(bc.to_mask()) {
                                        if fail_fast {
                                            return (0, false, true);
                                        }
                                        had_errors = true;
                                        *c = '\u{FFFD}';
                                    }
                                }
                            } else {
                                let mut numeral_state = RtlNumeralState::Undecided;
                                for c in prior.iter_mut() {
                                    let bc = self.data.bidi_class(*c);
                                    if !MIDDLE_RTL_MASK.intersects(bc.to_mask()) {
                                        if fail_fast {
                                            return (0, false, true);
                                        }
                                        had_errors = true;
                                        *c = '\u{FFFD}';
                                    } else {
                                        match numeral_state {
                                            RtlNumeralState::Undecided => {
                                                if bc.is_european_number() {
                                                    numeral_state = RtlNumeralState::European;
                                                } else if bc.is_arabic_number() {
                                                    numeral_state = RtlNumeralState::Arabic;
                                                }
                                            }
                                            RtlNumeralState::European => {
                                                if bc.is_arabic_number() {
                                                    if fail_fast {
                                                        return (0, false, true);
                                                    }
                                                    had_errors = true;
                                                    *c = '\u{FFFD}';
                                                }
                                            }
                                            RtlNumeralState::Arabic => {
                                                if bc.is_european_number() {
                                                    if fail_fast {
                                                        return (0, false, true);
                                                    }
                                                    had_errors = true;
                                                    *c = '\u{FFFD}';
                                                }
                                            }
                                        }
                                    }
                                }
                                if (numeral_state == RtlNumeralState::European
                                    && last_bc.is_arabic_number())
                                    || (numeral_state == RtlNumeralState::Arabic
                                        && last_bc.is_european_number())
                                {
                                    if fail_fast {
                                        return (0, false, true);
                                    }
                                    had_errors = true;
                                    *last = '\u{FFFD}';
                                }
                            }
                            break;
                        } else {
                            // One-character label or label where
                            // everything after the first character
                            // is just non-spacing marks.
                            break;
                        }
                    }
                }
            }
        }

        (passthrough_up_to, is_bidi, had_errors)
    }