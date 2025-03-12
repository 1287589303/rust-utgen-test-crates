fn after_punycode_decode(
        &self,
        domain_buffer: &mut SmallVec<[char; 253]>,
        current_label_start: usize,
        label_buffer: &[char],
        deny_list_deny_dot: u128,
        fail_fast: bool,
        had_errors: &mut bool,
    ) -> bool {
        for c in self
            .data
            .normalize_validate(label_buffer.iter().copied())
            .map(|c| apply_ascii_deny_list_to_lower_cased_unicode(c, deny_list_deny_dot))
        {
            if c == '\u{FFFD}' {
                if fail_fast {
                    return true;
                }
                *had_errors = true;
            }
            domain_buffer.push(c);
        }
        let normalized = &mut domain_buffer[current_label_start..];
        if let Err(()) =
            normalized
                .iter_mut()
                .zip(label_buffer.iter())
                .try_for_each(|(norm_c, decoded_c)| {
                    if *norm_c == *decoded_c {
                        Ok(())
                    } else {
                        // Mark the first difference
                        *norm_c = '\u{FFFD}';
                        Err(())
                    }
                })
        {
            if fail_fast {
                return true;
            }
            *had_errors = true;
        }
        false
    }