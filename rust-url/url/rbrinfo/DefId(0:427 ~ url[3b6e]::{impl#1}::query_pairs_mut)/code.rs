pub fn query_pairs_mut(&mut self) -> form_urlencoded::Serializer<'_, UrlQuery<'_>> {
        let fragment = self.take_fragment();

        let query_start;
        if let Some(start) = self.query_start {
            debug_assert!(self.byte_at(start) == b'?');
            query_start = start as usize;
        } else {
            query_start = self.serialization.len();
            self.query_start = Some(to_u32(query_start).unwrap());
            self.serialization.push('?');
        }

        let query = UrlQuery {
            url: Some(self),
            fragment,
        };
        form_urlencoded::Serializer::for_suffix(query, query_start + "?".len())
    }