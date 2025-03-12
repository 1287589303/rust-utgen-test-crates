pub fn parse_cannot_be_a_base_path<'i>(&mut self, mut input: Input<'i>) -> Input<'i> {
        loop {
            let input_before_c = input.clone();
            match input.next_utf8() {
                Some(('?', _)) | Some(('#', _)) if self.context == Context::UrlParser => {
                    return input_before_c
                }
                Some((c, utf8_c)) => {
                    self.check_url_code_point(c, &input);
                    self.serialization
                        .extend(utf8_percent_encode(utf8_c, CONTROLS));
                }
                None => return input,
            }
        }
    }