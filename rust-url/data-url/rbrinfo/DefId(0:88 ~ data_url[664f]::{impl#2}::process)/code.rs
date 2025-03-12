pub fn process(input: &'a str) -> Result<Self, DataUrlError> {
        use crate::DataUrlError::*;

        let after_colon = pretend_parse_data_url(input).ok_or(NotADataUrl)?;

        let (from_colon_to_comma, encoded_body_plus_fragment) =
            find_comma_before_fragment(after_colon).ok_or(NoComma)?;

        let (mime_type, base64) = parse_header(from_colon_to_comma);

        Ok(DataUrl {
            mime_type,
            base64,
            encoded_body_plus_fragment,
        })
    }