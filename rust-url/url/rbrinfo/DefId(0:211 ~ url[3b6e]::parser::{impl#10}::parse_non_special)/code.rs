fn parse_non_special(
        mut self,
        input: Input<'_>,
        scheme_type: SchemeType,
        scheme_end: u32,
    ) -> ParseResult<Url> {
        // path or authority state (
        if let Some(input) = input.split_prefix("//") {
            return self.after_double_slash(input, scheme_type, scheme_end);
        }
        // Anarchist URL (no authority)
        let path_start = to_u32(self.serialization.len())?;
        let username_end = path_start;
        let host_start = path_start;
        let host_end = path_start;
        let host = HostInternal::None;
        let port = None;
        let remaining = if let Some(input) = input.split_prefix('/') {
            self.serialization.push('/');
            self.parse_path(scheme_type, &mut false, path_start as usize, input)
        } else {
            self.parse_cannot_be_a_base_path(input)
        };
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