fn parse_query_and_fragment(
        &mut self,
        scheme_type: SchemeType,
        scheme_end: u32,
        mut input: Input<'_>,
    ) -> ParseResult<(Option<u32>, Option<u32>)> {
        let mut query_start = None;
        match input.next() {
            Some('#') => {}
            Some('?') => {
                query_start = Some(to_u32(self.serialization.len())?);
                self.serialization.push('?');
                let remaining = self.parse_query(scheme_type, scheme_end, input);
                if let Some(remaining) = remaining {
                    input = remaining
                } else {
                    return Ok((query_start, None));
                }
            }
            None => return Ok((None, None)),
            _ => panic!("Programming error. parse_query_and_fragment() called without ? or #"),
        }

        let fragment_start = to_u32(self.serialization.len())?;
        self.serialization.push('#');
        self.parse_fragment(input);
        Ok((query_start, Some(fragment_start)))
    }