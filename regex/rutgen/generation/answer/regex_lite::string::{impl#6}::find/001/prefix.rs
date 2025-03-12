// Answer 0

#[test]
fn test_find_match_exist() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = "I categorically deny having triskaidekaphobia.";
    let mat = re.find(hay).unwrap();
}

#[test]
fn test_find_no_match() {
    let re = Regex::new(r"\b\w{20}\b").unwrap();
    let hay = "Short haystack";
    let mat = re.find(hay);
}

#[test]
fn test_find_multiple_matches() {
    let re = Regex::new(r"\b\w{3}\b").unwrap();
    let hay = "Cat dog fish bat ant";
    let mat1 = re.find(hay).unwrap();
    let mat2 = re.find(&hay[mat1.end..]).unwrap();
}

#[test]
fn test_find_special_characters() {
    let re = Regex::new(r"\b\w{5}\b").unwrap();
    let hay = "hello! how are you? fine.";
    let mat = re.find(hay).unwrap();
}

#[test]
fn test_find_empty_haystack() {
    let re = Regex::new(r"\b\w{1}\b").unwrap();
    let hay = "";
    let mat = re.find(hay);
}

#[test]
fn test_find_haystack_with_whitespace() {
    let re = Regex::new(r"\b\w{6}\b").unwrap();
    let hay = "      Elon Musk is ";
    let mat = re.find(hay).unwrap();
}

#[test]
fn test_find_pattern_at_start() {
    let re = Regex::new(r"^cat").unwrap();
    let hay = "catapult is fun";
    let mat = re.find(hay).unwrap();
}

#[test]
fn test_find_pattern_at_end() {
    let re = Regex::new(r"dog$").unwrap();
    let hay = "The quick brown fox jumps over the lazy dog";
    let mat = re.find(hay).unwrap();
}

#[test]
fn test_find_pattern_length_boundary() {
    let re = Regex::new(r"\b\w{20}\b").unwrap();
    let hay = "wordwithexactlytwentycharacters";
    let mat = re.find(hay).unwrap();
}

#[test]
fn test_find_large_haystack() {
    let re = Regex::new(r"\d{3}").unwrap();
    let hay = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. 123 is a number.";
    let mat = re.find(hay).unwrap();
}

