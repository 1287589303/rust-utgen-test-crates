pub fn set_href(url: &mut Url, value: &str) -> Result<(), ParseError> {
    *url = Url::parse(value)?;
    Ok(())
}