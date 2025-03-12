pub fn set_hostname(url: &mut Url, new_hostname: &str) -> Result<(), ()> {
    if url.cannot_be_a_base() {
        return Err(());
    }
    // Host parsing rules are strict we don't want to trim the input
    let input = Input::new_no_trim(new_hostname);
    let scheme_type = SchemeType::from(url.scheme());
    if scheme_type == SchemeType::File && new_hostname.is_empty() {
        url.set_host_internal(Host::Domain(String::new()), None);
        return Ok(());
    }

    if let Ok((host, _remaining)) = Parser::parse_host(input, scheme_type) {
        if let Host::Domain(h) = &host {
            if h.is_empty() {
                // Empty host on special not file url
                if SchemeType::from(url.scheme()) == SchemeType::SpecialNotFile
                    // Port with an empty host
                    ||!port(url).is_empty()
                    // Empty host that includes credentials
                    || !url.username().is_empty()
                    || !url.password().unwrap_or("").is_empty()
                {
                    return Err(());
                }
            }
        }
        url.set_host_internal(host, None);
        Ok(())
    } else {
        Err(())
    }
}