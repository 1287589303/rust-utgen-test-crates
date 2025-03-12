fn after_double_slash(
        mut self,
        input: Input<'_>,
        scheme_type: SchemeType,
        scheme_end: u32,
    ) -> ParseResult<Url> {
        self.serialization.push('/');
        self.serialization.push('/');
        // authority state
        let before_authority = self.serialization.len();
        let (username_end, remaining) = self.parse_userinfo(input, scheme_type)?;
        let has_authority = before_authority != self.serialization.len();
        // host state
        let host_start = to_u32(self.serialization.len())?;
        let (host_end, host, port, remaining) =
            self.parse_host_and_port(remaining, scheme_end, scheme_type)?;
        if host == HostInternal::None && has_authority {
            return Err(ParseError::EmptyHost);
        }
        // path state
        let path_start = to_u32(self.serialization.len())?;
        let remaining = self.parse_path_start(scheme_type, &mut true, remaining);
        self.with_query_and_fragment(
            scheme_type,
            scheme_end,
            username_end,
            host_start,
            host_end,
            host,
            port,
            path_start,
            remaining,
        )
    }