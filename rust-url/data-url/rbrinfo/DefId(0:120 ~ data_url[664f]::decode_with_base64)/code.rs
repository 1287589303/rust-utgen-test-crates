fn decode_with_base64<F, E>(
    encoded_body_plus_fragment: &str,
    write_bytes: F,
) -> Result<Option<FragmentIdentifier<'_>>, forgiving_base64::DecodeError<E>>
where
    F: FnMut(&[u8]) -> Result<(), E>,
{
    let mut decoder = forgiving_base64::Decoder::new(write_bytes);
    let fragment = decode_without_base64(encoded_body_plus_fragment, |bytes| decoder.feed(bytes))?;
    decoder.finish()?;
    Ok(fragment)
}