fn get_file_host(input: Input<'_>) -> ParseResult<(Host<String>, Input<'_>)> {
        let (_, host_str, remaining) = Parser::file_host(input)?;
        let host = match Host::parse(&host_str)? {
            Host::Domain(ref d) if d == "localhost" => Host::Domain("".to_string()),
            host => host,
        };
        Ok((host, remaining))
    }