pub fn process<W: Write + ?Sized, OutputUnicode: FnMut(&[char], &[char], bool) -> bool>(
        &self,
        domain_name: &[u8],
        ascii_deny_list: AsciiDenyList,
        hyphens: Hyphens,
        error_policy: ErrorPolicy,
        mut output_as_unicode: OutputUnicode,
        sink: &mut W,
        ascii_sink: Option<&mut W>,
    ) -> Result<ProcessingSuccess, ProcessingError> {
        let fail_fast = error_policy == ErrorPolicy::FailFast;
        let mut domain_buffer = SmallVec::<[char; 253]>::new();
        let mut already_punycode = SmallVec::<[AlreadyAsciiLabel; 8]>::new();
        // `process_inner` could be pasted inline here, but it's out of line in order
        // to avoid duplicating that code when monomorphizing over `W` and `OutputUnicode`.
        let (passthrough_up_to, is_bidi, had_errors) = self.process_inner(
            domain_name,
            ascii_deny_list,
            hyphens,
            fail_fast,
            &mut domain_buffer,
            &mut already_punycode,
        );
        if passthrough_up_to == domain_name.len() {
            debug_assert!(!had_errors);
            return Ok(ProcessingSuccess::Passthrough);
        }
        // Checked only after passthrough as a micro optimization.
        if fail_fast && had_errors {
            return Err(ProcessingError::ValidityError);
        }
        debug_assert_eq!(had_errors, domain_buffer.contains(&'\u{FFFD}'));
        let without_dot = if let Some(without_dot) = domain_buffer.strip_suffix(&['.']) {
            without_dot
        } else {
            &domain_buffer[..]
        };
        // unwrap is OK, because we always have at least one label
        let tld = without_dot.rsplit(|c| *c == '.').next().unwrap();
        let mut had_unicode_output = false;
        let mut seen_label = false;
        let mut already_punycode_iter = already_punycode.iter();
        let mut passthrough_up_to_extended = passthrough_up_to;
        let mut flushed_prefix = false;
        for label in domain_buffer.split(|c| *c == '.') {
            // Unwrap is OK, because there are supposed to be as many items in
            // `already_punycode` as there are labels.
            let input_punycode = *already_punycode_iter.next().unwrap();
            if seen_label {
                if flushed_prefix {
                    sink.write_char('.')?;
                } else {
                    debug_assert_eq!(domain_name[passthrough_up_to_extended], b'.');
                    passthrough_up_to_extended += 1;
                    if passthrough_up_to_extended == domain_name.len() {
                        debug_assert!(!had_errors);
                        return Ok(ProcessingSuccess::Passthrough);
                    }
                }
            }
            seen_label = true;

            if let AlreadyAsciiLabel::MixedCaseAscii(mixed_case) = input_punycode {
                if let Some(first_upper_case) =
                    mixed_case.iter().position(|c| c.is_ascii_uppercase())
                {
                    let (head, tail) = mixed_case.split_at(first_upper_case);
                    let slice_to_write = if flushed_prefix {
                        head
                    } else {
                        flushed_prefix = true;
                        passthrough_up_to_extended += head.len();
                        debug_assert_ne!(passthrough_up_to_extended, domain_name.len());
                        &domain_name[..passthrough_up_to_extended]
                    };
                    // SAFETY: `mixed_case` and `domain_name` up to `passthrough_up_to_extended` are known to be ASCII.
                    sink.write_str(unsafe { core::str::from_utf8_unchecked(slice_to_write) })?;
                    for c in tail.iter() {
                        sink.write_char(char::from(c.to_ascii_lowercase()))?;
                    }
                } else if flushed_prefix {
                    // SAFETY: `mixed_case` is known to be ASCII.
                    sink.write_str(unsafe { core::str::from_utf8_unchecked(mixed_case) })?;
                } else {
                    passthrough_up_to_extended += mixed_case.len();
                    if passthrough_up_to_extended == domain_name.len() {
                        debug_assert!(!had_errors);
                        return Ok(ProcessingSuccess::Passthrough);
                    }
                }
                continue;
            }

            let potentially_punycode = if fail_fast {
                debug_assert!(classify_for_punycode(label) != PunycodeClassification::Error);
                !is_ascii(label)
            } else {
                classify_for_punycode(label) == PunycodeClassification::Unicode
            };
            let passthrough = if potentially_punycode {
                let unicode = output_as_unicode(label, tld, is_bidi);
                had_unicode_output |= unicode;
                unicode
            } else {
                true
            };
            if passthrough {
                if !flushed_prefix {
                    flushed_prefix = true;
                    // SAFETY: `domain_name` up to `passthrough_up_to_extended` is known to be ASCII.
                    sink.write_str(unsafe {
                        core::str::from_utf8_unchecked(&domain_name[..passthrough_up_to_extended])
                    })?;
                }
                for c in label.iter().copied() {
                    sink.write_char(c)?;
                }
            } else if let AlreadyAsciiLabel::MixedCasePunycode(mixed_case) = input_punycode {
                if let Some(first_upper_case) =
                    mixed_case.iter().position(|c| c.is_ascii_uppercase())
                {
                    let (head, tail) = mixed_case.split_at(first_upper_case);
                    let slice_to_write = if flushed_prefix {
                        head
                    } else {
                        flushed_prefix = true;
                        passthrough_up_to_extended += head.len();
                        debug_assert_ne!(passthrough_up_to_extended, domain_name.len());
                        &domain_name[..passthrough_up_to_extended]
                    };
                    // SAFETY: `mixed_case` and `domain_name` up to `passthrough_up_to_extended` are known to be ASCII.
                    sink.write_str(unsafe { core::str::from_utf8_unchecked(slice_to_write) })?;
                    for c in tail.iter() {
                        sink.write_char(char::from(c.to_ascii_lowercase()))?;
                    }
                } else if flushed_prefix {
                    // SAFETY: `mixed_case` is known to be ASCII.
                    sink.write_str(unsafe { core::str::from_utf8_unchecked(mixed_case) })?;
                } else {
                    passthrough_up_to_extended += mixed_case.len();
                    if passthrough_up_to_extended == domain_name.len() {
                        debug_assert!(!had_errors);
                        return Ok(ProcessingSuccess::Passthrough);
                    }
                }
            } else {
                if !flushed_prefix {
                    flushed_prefix = true;
                    // SAFETY: `domain_name` up to `passthrough_up_to_extended` is known to be ASCII.
                    sink.write_str(unsafe {
                        core::str::from_utf8_unchecked(&domain_name[..passthrough_up_to_extended])
                    })?;
                }
                write_punycode_label(label, sink)?;
            }
        }

        if had_errors {
            return Err(ProcessingError::ValidityError);
        }

        if had_unicode_output {
            if let Some(sink) = ascii_sink {
                let mut seen_label = false;
                let mut already_punycode_iter = already_punycode.iter();
                let mut passthrough_up_to_extended = passthrough_up_to;
                let mut flushed_prefix = false;
                for label in domain_buffer.split(|c| *c == '.') {
                    // Unwrap is OK, because there are supposed to be as many items in
                    // `already_punycode` as there are labels.
                    let input_punycode = *already_punycode_iter.next().unwrap();
                    if seen_label {
                        if flushed_prefix {
                            sink.write_char('.')?;
                        } else {
                            debug_assert_eq!(domain_name[passthrough_up_to_extended], b'.');
                            passthrough_up_to_extended += 1;
                        }
                    }
                    seen_label = true;

                    if let AlreadyAsciiLabel::MixedCaseAscii(mixed_case) = input_punycode {
                        if let Some(first_upper_case) =
                            mixed_case.iter().position(|c| c.is_ascii_uppercase())
                        {
                            let (head, tail) = mixed_case.split_at(first_upper_case);
                            let slice_to_write = if flushed_prefix {
                                head
                            } else {
                                flushed_prefix = true;
                                passthrough_up_to_extended += head.len();
                                debug_assert_ne!(passthrough_up_to_extended, domain_name.len());
                                &domain_name[..passthrough_up_to_extended]
                            };
                            // SAFETY: `mixed_case` and `domain_name` up to `passthrough_up_to_extended` are known to be ASCII.
                            sink.write_str(unsafe {
                                core::str::from_utf8_unchecked(slice_to_write)
                            })?;
                            for c in tail.iter() {
                                sink.write_char(char::from(c.to_ascii_lowercase()))?;
                            }
                        } else if flushed_prefix {
                            // SAFETY: `mixed_case` is known to be ASCII.
                            sink.write_str(unsafe { core::str::from_utf8_unchecked(mixed_case) })?;
                        } else {
                            passthrough_up_to_extended += mixed_case.len();
                        }
                        continue;
                    }

                    if is_ascii(label) {
                        if !flushed_prefix {
                            flushed_prefix = true;
                            // SAFETY: `domain_name` up to `passthrough_up_to_extended` is known to be ASCII.
                            sink.write_str(unsafe {
                                core::str::from_utf8_unchecked(
                                    &domain_name[..passthrough_up_to_extended],
                                )
                            })?;
                        }
                        for c in label.iter().copied() {
                            sink.write_char(c)?;
                        }
                    } else if let AlreadyAsciiLabel::MixedCasePunycode(mixed_case) = input_punycode
                    {
                        if let Some(first_upper_case) =
                            mixed_case.iter().position(|c| c.is_ascii_uppercase())
                        {
                            let (head, tail) = mixed_case.split_at(first_upper_case);
                            let slice_to_write = if flushed_prefix {
                                head
                            } else {
                                flushed_prefix = true;
                                passthrough_up_to_extended += head.len();
                                debug_assert_ne!(passthrough_up_to_extended, domain_name.len());
                                &domain_name[..passthrough_up_to_extended]
                            };
                            // SAFETY: `mixed_case` and `domain_name` up to `passthrough_up_to_extended` are known to be ASCII.
                            sink.write_str(unsafe {
                                core::str::from_utf8_unchecked(slice_to_write)
                            })?;
                            for c in tail.iter() {
                                sink.write_char(char::from(c.to_ascii_lowercase()))?;
                            }
                        } else if flushed_prefix {
                            // SAFETY: `mixed_case` is known to be ASCII.
                            sink.write_str(unsafe { core::str::from_utf8_unchecked(mixed_case) })?;
                        } else {
                            passthrough_up_to_extended += mixed_case.len();
                        }
                    } else {
                        if !flushed_prefix {
                            flushed_prefix = true;
                            // SAFETY: `domain_name` up to `passthrough_up_to_extended` is known to be ASCII.
                            sink.write_str(unsafe {
                                core::str::from_utf8_unchecked(
                                    &domain_name[..passthrough_up_to_extended],
                                )
                            })?;
                        }
                        write_punycode_label(label, sink)?;
                    }
                }
                if !flushed_prefix {
                    // SAFETY: `domain_name` up to `passthrough_up_to_extended` is known to be ASCII.
                    sink.write_str(unsafe {
                        core::str::from_utf8_unchecked(&domain_name[..passthrough_up_to_extended])
                    })?;
                }
            }
        }
        Ok(ProcessingSuccess::WroteToSink)
    }