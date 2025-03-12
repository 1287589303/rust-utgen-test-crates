// Answer 0

#[test]
fn test_extract_valid_case() {
    use regex::bytes::Regex;

    let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
    let hay = b"On 2010-03-14, I became a Tenneessee lamb.";
    let caps = re.captures(hay).unwrap();
    
    let (full, [year, month, day]) = caps.extract::<3>();
}

#[test]
fn test_extract_boundary_case() {
    use regex::bytes::Regex;

    let re = Regex::new(r"([a-zA-Z]{3})([0-9]{3})([!@#$%^&*()])").unwrap();
    let hay = b"abc123$ def456& ghi789*";
    let caps = re.captures(hay).unwrap();
    
    let (full, [text, num, symbol]) = caps.extract::<3>();
} 

#[test]
#[should_panic]
fn test_extract_insufficient_groups() {
    use regex::bytes::Regex;

    let re = Regex::new(r"([a-z]+)([0-9]+)").unwrap();
    let hay = b"abc123";
    let caps = re.captures(hay).unwrap();

    let (_, [text]) = caps.extract::<2>();
}

#[test]
#[should_panic]
fn test_extract_excess_groups() {
    use regex::bytes::Regex;

    let re = Regex::new(r"([1-9])([0-9]{2})([0-9]{2})").unwrap();
    let hay = b"123456";
    let caps = re.captures(hay).unwrap();

    let (_, [first, second, third]) = caps.extract::<4>();
} 

#[test]
fn test_extract_varied_content() {
    use regex::bytes::Regex;

    let re = Regex::new(r"(\w+)-(\d+)-(\w+)").unwrap();
    let hay = b"date-2023-rust";
    let caps = re.captures(hay).unwrap();

    let (full, [word, number, lang]) = caps.extract::<3>();
}

