pub fn decode_to_vec(
        &self,
    ) -> Result<(Vec<u8>, Option<FragmentIdentifier<'a>>), forgiving_base64::InvalidBase64> {
        let mut body = Vec::new();
        let fragment = self.decode(|bytes| {
            body.extend_from_slice(bytes);
            Ok(())
        })?;
        Ok((body, fragment))
    }