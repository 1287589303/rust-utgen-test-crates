pub fn to_user_interface<'a, OutputUnicode: FnMut(&[char], &[char], bool) -> bool>(
        &self,
        domain_name: &'a [u8],
        ascii_deny_list: AsciiDenyList,
        hyphens: Hyphens,
        output_as_unicode: OutputUnicode,
    ) -> (Cow<'a, str>, Result<(), crate::Errors>) {
        let mut s = String::new();
        match self.process(
            domain_name,
            ascii_deny_list,
            hyphens,
            ErrorPolicy::MarkErrors,
            output_as_unicode,
            &mut s,
            None,
        ) {
            // SAFETY: `ProcessingSuccess::Passthrough` asserts that `domain_name` is ASCII.
            Ok(ProcessingSuccess::Passthrough) => (
                Cow::Borrowed(unsafe { core::str::from_utf8_unchecked(domain_name) }),
                Ok(()),
            ),
            Ok(ProcessingSuccess::WroteToSink) => (Cow::Owned(s), Ok(())),
            Err(ProcessingError::ValidityError) => (Cow::Owned(s), Err(crate::Errors::default())),
            Err(ProcessingError::SinkError) => unreachable!(),
        }
    }