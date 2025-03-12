fn contains(parameters: &[(String, String)], name: &str) -> bool {
    parameters.iter().any(|(n, _)| n == name)
}