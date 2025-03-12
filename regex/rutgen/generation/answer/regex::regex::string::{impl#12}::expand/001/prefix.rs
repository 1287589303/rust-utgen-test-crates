// Answer 0

#[test]
fn test_expand_valid_capture_groups() {
    let re = Regex::new(r"(?<day>[0-9]{2})-(?<month>[0-9]{2})-(?<year>[0-9]{4})").unwrap();
    let hay = "On 14-03-2010, I became a Tennessee lamb.";
    let caps = re.captures(hay).unwrap();

    let mut dst = String::new();
    caps.expand("year=${year}, month=${month}, day=${day}", &mut dst);
}

#[test]
fn test_expand_with_empty_replacement() {
    let re = Regex::new(r"(?<day>[0-9]{2})-(?<month>[0-9]{2})-(?<year>[0-9]{4})").unwrap();
    let hay = "On 14-03-2010, I became a Tennessee lamb.";
    let caps = re.captures(hay).unwrap();

    let mut dst = String::new();
    caps.expand("", &mut dst);
}

#[test]
fn test_expand_with_single_capture_group() {
    let re = Regex::new(r"(?<number>[0-9]{2})").unwrap();
    let hay = "The number is 42.";
    let caps = re.captures(hay).unwrap();

    let mut dst = String::new();
    caps.expand("Number: $number", &mut dst);
}

#[test]
fn test_expand_with_multiple_capture_groups() {
    let re = Regex::new(r"(?<name>\w+) (?<age>[0-9]+)").unwrap();
    let hay = "Alice 30";
    let caps = re.captures(hay).unwrap();

    let mut dst = String::new();
    caps.expand("${name} is ${age} years old.", &mut dst);
}

#[test]
fn test_expand_out_of_bounds_index() {
    let re = Regex::new(r"(a)(b)(c)").unwrap();
    let hay = "abc";
    let caps = re.captures(hay).unwrap();

    let mut dst = String::new();
    caps.expand("$3$4", &mut dst);
}

#[test]
fn test_expand_with_special_characters() {
    let re = Regex::new(r"(?<key>[A-Za-z]+): (?<value>\d+)").unwrap();
    let hay = "X: 10";
    let caps = re.captures(hay).unwrap();

    let mut dst = String::new();
    caps.expand("${key} costs $value bucks.", &mut dst);
}

