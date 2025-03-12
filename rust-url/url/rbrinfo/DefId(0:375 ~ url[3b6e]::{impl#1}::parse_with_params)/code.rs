pub fn parse_with_params<I, K, V>(input: &str, iter: I) -> Result<Url, crate::ParseError>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut url = Url::options().parse(input);

        if let Ok(ref mut url) = url {
            url.query_pairs_mut().extend_pairs(iter);
        }

        url
    }