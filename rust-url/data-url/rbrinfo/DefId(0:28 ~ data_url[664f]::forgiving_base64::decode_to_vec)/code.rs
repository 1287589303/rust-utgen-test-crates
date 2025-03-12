pub fn decode_to_vec(input: &[u8]) -> Result<Vec<u8>, InvalidBase64> {
    let mut v = Vec::new();
    {
        let mut decoder = Decoder::new(|bytes| {
            v.extend_from_slice(bytes);
            Ok(())
        });
        decoder.feed(input)?;
        decoder.finish()?;
    }
    Ok(v)
}