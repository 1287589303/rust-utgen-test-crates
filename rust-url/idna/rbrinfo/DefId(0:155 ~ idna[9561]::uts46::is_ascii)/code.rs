fn is_ascii(label: &[char]) -> bool {
    for c in label.iter() {
        if !c.is_ascii() {
            return false;
        }
    }
    true
}