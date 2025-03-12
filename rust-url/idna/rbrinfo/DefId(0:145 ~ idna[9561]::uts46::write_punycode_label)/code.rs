fn write_punycode_label<W: Write + ?Sized>(
    label: &[char],
    sink: &mut W,
) -> Result<(), ProcessingError> {
    sink.write_str("xn--")?;
    crate::punycode::encode_into::<_, _, InternalCaller>(label.iter().copied(), sink)?;
    Ok(())
}