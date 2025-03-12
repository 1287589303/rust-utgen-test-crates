pub fn to_ascii<'a>(
        &self,
        domain_name: &'a [u8],
        ascii_deny_list: AsciiDenyList,
        hyphens: Hyphens,
        dns_length: DnsLength,
    ) -> Result<Cow<'a, str>, crate::Errors> {
        let mut s = String::new();
        match self.process(
            domain_name,
            ascii_deny_list,
            hyphens,
            ErrorPolicy::FailFast,
            |_, _, _| false,
            &mut s,
            None,
        ) {
            // SAFETY: `ProcessingSuccess::Passthrough` asserts that `domain_name` is ASCII.
            Ok(ProcessingSuccess::Passthrough) => {
                let cow = Cow::Borrowed(unsafe { core::str::from_utf8_unchecked(domain_name) });
                if dns_length != DnsLength::Ignore
                    && !verify_dns_length(&cow, dns_length == DnsLength::VerifyAllowRootDot)
                {
                    Err(crate::Errors::default())
                } else {
                    Ok(cow)
                }
            }
            Ok(ProcessingSuccess::WroteToSink) => {
                let cow: Cow<'_, str> = Cow::Owned(s);
                if dns_length != DnsLength::Ignore
                    && !verify_dns_length(&cow, dns_length == DnsLength::VerifyAllowRootDot)
                {
                    Err(crate::Errors::default())
                } else {
                    Ok(cow)
                }
            }
            Err(ProcessingError::ValidityError) => Err(crate::Errors::default()),
            Err(ProcessingError::SinkError) => unreachable!(),
        }
    }