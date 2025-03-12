fn parse_flag(
        &self,
        flags: &mut Flags,
        negate: bool,
    ) -> Result<(), Error> {
        let enabled = !negate;
        match self.char() {
            'i' => flags.case_insensitive = enabled,
            'm' => flags.multi_line = enabled,
            's' => flags.dot_matches_new_line = enabled,
            'U' => flags.swap_greed = enabled,
            'R' => flags.crlf = enabled,
            'x' => flags.ignore_whitespace = enabled,
            // We make a special exception for this flag where we let it
            // through as a recognized flag, but treat it as a no-op. This in
            // practice retains some compatibility with the regex crate. It is
            // a little suspect to do this, but for example, '(?-u:\b).+' in
            // the regex crate is equivalent to '\b.+' in regex-lite.
            'u' => {}
            _ => return Err(Error::new(ERR_FLAG_UNRECOGNIZED)),
        }
        Ok(())
    }