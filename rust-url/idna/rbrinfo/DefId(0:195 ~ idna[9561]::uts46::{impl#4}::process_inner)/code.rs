fn process_inner<'a>(
        &self,
        domain_name: &'a [u8],
        ascii_deny_list: AsciiDenyList,
        hyphens: Hyphens,
        fail_fast: bool,
        domain_buffer: &mut SmallVec<[char; 253]>,
        already_punycode: &mut SmallVec<[AlreadyAsciiLabel<'a>; 8]>,
    ) -> (usize, bool, bool) {
        // Sadly, this even faster-path ASCII tier is needed to avoid regressing
        // performance.
        let mut iter = domain_name.iter();
        let mut most_recent_label_start = iter.clone();
        loop {
            if let Some(&b) = iter.next() {
                if in_inclusive_range8(b, b'a', b'z') {
                    continue;
                }
                if b == b'.' {
                    most_recent_label_start = iter.clone();
                    continue;
                }
                return self.process_innermost(
                    domain_name,
                    ascii_deny_list,
                    hyphens,
                    fail_fast,
                    domain_buffer,
                    already_punycode,
                    most_recent_label_start.as_slice(),
                );
            } else {
                // Success! The whole input passes through on the fastest path!
                return (domain_name.len(), false, false);
            }
        }
    }