pub fn to_ascii(&mut self, domain: &str, out: &mut String) -> Result<(), Errors> {
        let mapped = map_transitional(domain, self.config.transitional_processing);
        match Uts46::new().process(
            mapped.as_bytes(),
            self.config.deny_list(),
            self.config.hyphens(),
            ErrorPolicy::FailFast, // Old code did not appear to expect the output to be useful in the error case.
            |_, _, _| false,
            out,
            None,
        ) {
            Ok(ProcessingSuccess::Passthrough) => {
                if self.config.verify_dns_length && !verify_dns_length(&mapped, true) {
                    return Err(crate::Errors::default());
                }
                out.push_str(&mapped);
                Ok(())
            }
            Ok(ProcessingSuccess::WroteToSink) => {
                if self.config.verify_dns_length && !verify_dns_length(out, true) {
                    return Err(crate::Errors::default());
                }
                Ok(())
            }
            Err(ProcessingError::ValidityError) => Err(crate::Errors::default()),
            Err(ProcessingError::SinkError) => unreachable!(),
        }
    }