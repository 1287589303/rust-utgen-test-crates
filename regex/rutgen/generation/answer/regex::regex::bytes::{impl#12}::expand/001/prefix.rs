// Answer 0

#[test]
fn test_expand_valid_unbraced_capture() {
    let hay: &[u8] = b"On 14-03-2010, I became a Tenneessee lamb.";
    let re = regex::bytes::Regex::new(
        r"(?<day>[0-9]{2})-(?<month>[0-9]{2})-(?<year>[0-9]{4})",
    ).unwrap();
    let caps = re.captures(hay).unwrap();

    let mut dst = Vec::new();
    caps.expand(b"year=$year, month=$month, day=$day", &mut dst);
}

#[test]
fn test_expand_valid_braced_capture() {
    let hay: &[u8] = b"On 14-03-2010, I became a Tenneessee lamb.";
    let re = regex::bytes::Regex::new(
        r"(?<day>[0-9]{2})-(?<month>[0-9]{2})-(?<year>[0-9]{4})",
    ).unwrap();
    let caps = re.captures(hay).unwrap();

    let mut dst = Vec::new();
    caps.expand(b"year=${year}, month=${month}, day=${day}", &mut dst);
}

#[test]
fn test_expand_invalid_capture() {
    let hay: &[u8] = b"On 14-03-2010, I became a Tenneessee lamb.";
    let re = regex::bytes::Regex::new(
        r"(?<day>[0-9]{2})-(?<month>[0-9]{2})-(?<year>[0-9]{4})",
    ).unwrap();
    let caps = re.captures(hay).unwrap();

    let mut dst = Vec::new();
    caps.expand(b"year=$99, month=$invalid, day=${invalid}", &mut dst);
}

#[test]
fn test_expand_multiple_captures() {
    let hay: &[u8] = b"On 14-03-2010, I became a Tenneessee lamb.";
    let re = regex::bytes::Regex::new(
        r"(?<day>[0-9]{2})-(?<month>[0-9]{2})-(?<year>[0-9]{4})",
    ).unwrap();
    let caps = re.captures(hay).unwrap();

    let mut dst = Vec::new();
    caps.expand(b"$0$1$2", &mut dst);
}

#[test]
fn test_expand_literal_dollar() {
    let hay: &[u8] = b"On 14-03-2010, I became a Tenneessee lamb.";
    let re = regex::bytes::Regex::new(
        r"(?<day>[0-9]{2})-(?<month>[0-9]{2})-(?<year>[0-9]{4})",
    ).unwrap();
    let caps = re.captures(hay).unwrap();

    let mut dst = Vec::new();
    caps.expand(b"$$", &mut dst);
}

#[test]
fn test_expand_empty_replacement() {
    let hay: &[u8] = b"On 14-03-2010, I became a Tenneessee lamb.";
    let re = regex::bytes::Regex::new(
        r"(?<day>[0-9]{2})-(?<month>[0-9]{2})-(?<year>[0-9]{4})",
    ).unwrap();
    let caps = re.captures(hay).unwrap();

    let mut dst = Vec::new();
    caps.expand(b"", &mut dst);
}

#[test]
fn test_expand_single_character_replacement() {
    let hay: &[u8] = b"On 14-03-2010, I became a Tenneessee lamb.";
    let re = regex::bytes::Regex::new(
        r"(?<day>[0-9]{2})-(?<month>[0-9]{2})-(?<year>[0-9]{4})",
    ).unwrap();
    let caps = re.captures(hay).unwrap();

    let mut dst = Vec::new();
    caps.expand(b"x", &mut dst);
}

