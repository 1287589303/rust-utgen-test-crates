fn with_query_and_fragment(
        mut self,
        scheme_type: SchemeType,
        scheme_end: u32,
        username_end: u32,
        host_start: u32,
        host_end: u32,
        host: HostInternal,
        port: Option<u16>,
        mut path_start: u32,
        remaining: Input<'_>,
    ) -> ParseResult<Url> {
        // Special case for anarchist URL's with a leading empty path segment
        // This prevents web+demo:/.//not-a-host/ or web+demo:/path/..//not-a-host/,
        // when parsed and then serialized, from ending up as web+demo://not-a-host/
        // (they end up as web+demo:/.//not-a-host/).
        //
        // If url’s host is null, url does not have an opaque path,
        // url’s path’s size is greater than 1, and url’s path[0] is the empty string,
        // then append U+002F (/) followed by U+002E (.) to output.
        let scheme_end_as_usize = scheme_end as usize;
        let path_start_as_usize = path_start as usize;
        if path_start_as_usize == scheme_end_as_usize + 1 {
            // Anarchist URL
            if self.serialization[path_start_as_usize..].starts_with("//") {
                // Case 1: The base URL did not have an empty path segment, but the resulting one does
                // Insert the "/." prefix
                self.serialization.insert_str(path_start_as_usize, "/.");
                path_start += 2;
            }
            assert!(!self.serialization[scheme_end_as_usize..].starts_with("://"));
        } else if path_start_as_usize == scheme_end_as_usize + 3
            && &self.serialization[scheme_end_as_usize..path_start_as_usize] == ":/."
        {
            // Anarchist URL with leading empty path segment
            // The base URL has a "/." between the host and the path
            assert_eq!(self.serialization.as_bytes()[path_start_as_usize], b'/');
            if self
                .serialization
                .as_bytes()
                .get(path_start_as_usize + 1)
                .copied()
                != Some(b'/')
            {
                // Case 2: The base URL had an empty path segment, but the resulting one does not
                // Remove the "/." prefix
                self.serialization
                    .replace_range(scheme_end_as_usize..path_start_as_usize, ":");
                path_start -= 2;
            }
            assert!(!self.serialization[scheme_end_as_usize..].starts_with("://"));
        }

        let (query_start, fragment_start) =
            self.parse_query_and_fragment(scheme_type, scheme_end, remaining)?;
        Ok(Url {
            serialization: self.serialization,
            scheme_end,
            username_end,
            host_start,
            host_end,
            host,
            port,
            path_start,
            query_start,
            fragment_start,
        })
    }