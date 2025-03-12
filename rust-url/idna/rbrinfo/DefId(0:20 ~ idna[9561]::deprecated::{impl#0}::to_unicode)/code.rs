pub fn to_unicode(&mut self, domain: &str, out: &mut String) -> Result<(), Errors> {
        let mapped = map_transitional(domain, self.config.transitional_processing);
        match Uts46::new().process(
            mapped.as_bytes(),
            self.config.deny_list(),
            self.config.hyphens(),
            ErrorPolicy::MarkErrors,
            |_, _, _| true,
            out,
            None,
        ) {
            Ok(ProcessingSuccess::Passthrough) => {
                out.push_str(&mapped);
                Ok(())
            }
            Ok(ProcessingSuccess::WroteToSink) => Ok(()),
            Err(ProcessingError::ValidityError) => Err(crate::Errors::default()),
            Err(ProcessingError::SinkError) => unreachable!(),
        }
    }