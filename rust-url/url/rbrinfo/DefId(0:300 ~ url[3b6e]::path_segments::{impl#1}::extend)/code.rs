pub fn extend<I>(&mut self, segments: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let scheme_type = SchemeType::from(self.url.scheme());
        let path_start = self.url.path_start as usize;
        self.url.mutate(|parser| {
            parser.context = parser::Context::PathSegmentSetter;
            for segment in segments {
                let segment = segment.as_ref();
                if matches!(segment, "." | "..") {
                    continue;
                }
                if parser.serialization.len() > path_start + 1
                    // Non special url's path might still be empty
                    || parser.serialization.len() == path_start
                {
                    parser.serialization.push('/');
                }
                let mut has_host = true; // FIXME account for this?
                parser.parse_path(
                    scheme_type,
                    &mut has_host,
                    path_start,
                    parser::Input::new_no_trim(segment),
                );
            }
        });
        self
    }