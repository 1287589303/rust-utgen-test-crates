fn parse_ipv4addr(input: &str) -> ParseResult<Ipv4Addr> {
    let mut parts: Vec<&str> = input.split('.').collect();
    if parts.last() == Some(&"") {
        parts.pop();
    }
    if parts.len() > 4 {
        return Err(ParseError::InvalidIpv4Address);
    }
    let mut numbers: Vec<u32> = Vec::new();
    for part in parts {
        match parse_ipv4number(part) {
            Ok(Some(n)) => numbers.push(n),
            Ok(None) => return Err(ParseError::InvalidIpv4Address), // u32 overflow
            Err(()) => return Err(ParseError::InvalidIpv4Address),
        };
    }
    let mut ipv4 = numbers.pop().expect("a non-empty list of numbers");
    // Equivalent to: ipv4 >= 256 ** (4 − numbers.len())
    if ipv4 > u32::MAX >> (8 * numbers.len() as u32) {
        return Err(ParseError::InvalidIpv4Address);
    }
    if numbers.iter().any(|x| *x > 255) {
        return Err(ParseError::InvalidIpv4Address);
    }
    for (counter, n) in numbers.iter().enumerate() {
        ipv4 += n << (8 * (3 - counter as u32))
    }
    Ok(Ipv4Addr::from(ipv4))
}