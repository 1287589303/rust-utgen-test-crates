fn parse_file_host<'i>(
        &mut self,
        input: Input<'i>,
    ) -> ParseResult<(bool, HostInternal, Input<'i>)> {
        let has_host;
        let (_, host_str, remaining) = Parser::file_host(input)?;
        let host = if host_str.is_empty() {
            has_host = false;
            HostInternal::None
        } else {
            match Host::parse(&host_str)? {
                Host::Domain(ref d) if d == "localhost" => {
                    has_host = false;
                    HostInternal::None
                }
                host => {
                    write!(&mut self.serialization, "{}", host).unwrap();
                    has_host = true;
                    host.into()
                }
            }
        };
        Ok((has_host, host, remaining))
    }