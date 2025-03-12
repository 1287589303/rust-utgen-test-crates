pub fn set_host(url: &mut Url, new_host: &str) -> Result<(), ()> {
    // If context object’s url’s cannot-be-a-base-URL flag is set, then return.
    if url.cannot_be_a_base() {
        return Err(());
    }
    // Host parsing rules are strict,
    // We don't want to trim the input
    let input = Input::new_no_trim(new_host);
    let host;
    let opt_port;
    {
        let scheme = url.scheme();
        let scheme_type = SchemeType::from(scheme);
        if scheme_type == SchemeType::File && new_host.is_empty() {
            url.set_host_internal(Host::Domain(String::new()), None);
            return Ok(());
        }

        if let Ok((h, remaining)) = Parser::parse_host(input, scheme_type) {
            host = h;
            opt_port = if let Some(remaining) = remaining.split_prefix(':') {
                if remaining.is_empty() {
                    None
                } else {
                    Parser::parse_port(remaining, || default_port(scheme), Context::Setter)
                        .ok()
                        .map(|(port, _remaining)| port)
                }
            } else {
                None
            };
        } else {
            return Err(());
        }
    }
    // Make sure we won't set an empty host to a url with a username or a port
    if host == Host::Domain("".to_string())
        && (!username(url).is_empty() || matches!(opt_port, Some(Some(_))) || url.port().is_some())
    {
        return Err(());
    }
    url.set_host_internal(host, opt_port);
    Ok(())
}