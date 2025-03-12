fn parse_host_and_port<'i>(
        &mut self,
        input: Input<'i>,
        scheme_end: u32,
        scheme_type: SchemeType,
    ) -> ParseResult<(u32, HostInternal, Option<u16>, Input<'i>)> {
        let (host, remaining) = Parser::parse_host(input, scheme_type)?;
        write!(&mut self.serialization, "{}", host).unwrap();
        let host_end = to_u32(self.serialization.len())?;
        if let Host::Domain(h) = &host {
            if h.is_empty() {
                // Port with an empty host
                if remaining.starts_with(":") {
                    return Err(ParseError::EmptyHost);
                }
                if scheme_type.is_special() {
                    return Err(ParseError::EmptyHost);
                }
            }
        };

        let (port, remaining) = if let Some(remaining) = remaining.split_prefix(':') {
            let scheme = || default_port(&self.serialization[..scheme_end as usize]);
            let (port, remaining) = Parser::parse_port(remaining, scheme, self.context)?;
            if let Some(port) = port {
                self.serialization.push(':');
                let mut buffer = [0u8; 5];
                let port_str = fast_u16_to_str(&mut buffer, port);
                self.serialization.push_str(port_str);
            }
            (port, remaining)
        } else {
            (None, remaining)
        };
        Ok((host_end, host.into(), port, remaining))
    }